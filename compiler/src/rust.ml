open Ast
type string = type_name

module Ctx = struct
  type t = {
    buf: Buffer.t;
  }
  
  let make size =
    { buf = Buffer.create size }
end

let aspf = Format.asprintf

let type_name_opt = function
  | None -> ""
  | Some typename -> aspf ": %s" typename

let literal = function
  | NumberLiteral (n, _) -> n
  | StringLiteral (str, _) -> aspf "\"%s\"" str
  | TrueLiteral _ -> "true"
  | FalseLiteral _ -> "false"

let rec expr = function
  | Identifier id -> id
  | Literal lit -> literal lit
  | FunctionCall (id, args) ->
      let args = List.map (expr ) args |> String.concat ", " in
      aspf {|%s(%s)|} id args

let typed_identifier (id,typename) =
  aspf {|%s%s|} id (type_name_opt typename)

let list f arg sep =
  List.map f arg |> String.concat sep

let notimpl id =
  aspf {| [- not implemented (%s) -] |} id

let rec statement  = function
  | Break -> "break"
  | Continue -> "continue"
  | Expression e -> expr  e
  | Block stmts -> 
      aspf {|{ %s }|} (statements  stmts)
  | VariableDeclaration (names, e) ->
      let names = list typed_identifier names ", " in
      let expr = match e with
      | None -> ";"
      | Some e -> aspf " := %s" @@ expr e in
      aspf "let mut %s%s" names expr
  | Assignment (ids, exp) ->
      let ids = String.concat ", " ids in
      let exp = expr exp in
      aspf {|%s := %s|} ids exp
  | If (exp, stmts) -> 
      let exp = expr exp in
      let stmts = statements stmts in
      aspf "if %s { %s }" exp stmts
  | Switch (exp, cases, _) ->
      let exp = expr exp in
      let case (lit, stmts) =
        aspf {|%s => {%s},|} (literal lit) @@ statements stmts
      in
      let cases = list case cases "\n" in
      aspf {|match %s {%s}|} exp cases
  (* Convert to while if it's empty *)
  | ForLoop _ -> notimpl "forloop"
  | FunctionDefinition _  -> notimpl "funcdef"
  | Leave -> notimpl "leave"
  
and statements stmts =
  list statement stmts ";\n"
