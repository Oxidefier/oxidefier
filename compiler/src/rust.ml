open Ast
open Utils
type string = type_name

module Ctx = struct
  type t = {
    fundec: string list option;
  }
  
  let make () =
    { fundec = None }

  let set_fundec _ctx ids =
  { fundec = Some ids }

end

let default_tyn = "U256"
let default_tyn_res = "Result<U256, ReturnOrRevert>"
let memory_param = "mem: &mut Memory"
let memory_arg = "mem"

let aspf = Format.asprintf

let prelude =
  {|#![allow(warnings)]
use alloy_primitives::U256;
mod opcodes;
use opcodes::*;
enum ReturnOrRevert {
      Return {start: U256, end: U256},
      Revert {start: U256, end: U256}
    }
fn to_bool(a: U256) -> Result<bool, ReturnOrRevert> {
  Ok(!(a == 0))
}
    |}
(**)
(* let main_fn = *)
(*   {|fn main() { *)
(*       let mut  *)
(*     } *)
(*     |} *)

let literal = function
  | NumberLiteral (n, _) -> aspf {|"%s".parse().unwrap()|}n
  | StringLiteral (str, _) -> aspf "\"%s\"" str
  | TrueLiteral _ -> "true"
  | FalseLiteral _ -> "false"

let rec expr = function
  | Identifier id -> id
  | Literal lit -> literal lit
  | FunctionCall (id, args) ->
      (* return is a evm function, which means somthing different
         in rust. *)
      (* TODO: make this change part of an ast pass*)
      let id = match id with
      | "return" -> "return_evm"
      | _ -> id in
      (* FIXME: this is a test. *)
      (* let id = match List.length args with *)
      (* | 0 -> "h" *)
      (* | 1 -> "f" *)
      (* | 2 -> "g" *)
      (* | _ -> id in *)
      let args = List.map (expr) args in
      let args = memory_arg :: args |> String.concat ", " in
      aspf {|%s(%s)?|} id args

let type_name_opt default_tyn = function
  | None -> aspf ": %s" default_tyn
  | Some typename -> aspf ": %s" typename


let typed_identifier default_tyn (id,typename) =
  aspf {|%s%s|} id (type_name_opt default_tyn typename)

let typed_identifier_flat arg =
  typed_identifier default_tyn arg

let typed_identifier_res arg =
  typed_identifier default_tyn_res arg

let list f arg sep =
  List.map f arg |> String.concat sep

let notimpl id =
  aspf {| [- not implemented (%s) -] |} id

let rec statement ctx = function
  | Break -> "break"
  | Continue -> "continue"
  | Expression e -> expr e
  | Block stmts -> 
      aspf {|{ %s }|} (statements ctx stmts)
  | VariableDeclaration (names, e) ->
      let names = list typed_identifier_flat names ", " in
      let expr = match e with
      | None -> ""
      | Some e -> aspf " = %s" @@ expr e in
      aspf "let mut %s%s" names expr
  | Assignment (ids, exp) ->
      let ids = String.concat ", " ids in
      let exp = expr exp in
      aspf {|%s = %s|} ids exp
  | If (exp, stmts) -> 
      let exp = expr exp in
      let stmts = statements ctx stmts in
      aspf "if to_bool(%s)? { %s }" exp stmts
  | Switch (exp, cases, _) ->
      let exp = expr exp in
      let case (lit, stmts) =
        aspf {|%s => {%s},|} (literal lit) @@ statements ctx stmts
      in
      let cases = list case cases "\n" in
      aspf {|match %s {%s}|} exp cases
  (* Convert to while if it's empty *)
  | ForLoop _ -> notimpl "forloop"
  | FunctionDefinition obj ->
      let mem_param = "mem: &mut Memory" in
      let params = match obj.params with
      | None -> mem_param
      | Some l ->
          let params = List.rmap l
            (fun ti -> aspf {|mut %s|} @@ typed_identifier_flat ti) in
          mem_param :: params |> String.concat ", " in
      let ret_ids = match obj.returns with
      | None -> []
      | Some l -> List.map fst l in
      (* Standard YUL only uses U256 *)
      let ret_tys = match obj.returns with
      | None -> []
      | Some l -> List.map (fun _ -> "U256") l in
      (* build the return type def *)
      let ret_tys = match ret_tys with
      | [] -> "()"
      | _::[] -> "U256"
      | l -> aspf "(%s)" @@ String.concat ", " l in
      let ret_ty =
        aspf "-> Result<%s, ReturnOrRevert>" ret_tys in
      (* Declare the return variables *)
      let body = 
        match ret_ids with
        | [] -> obj.body 
        | _ -> 
          let return_vars_dec =
          let typed_ids = List.map (fun id -> (id, None)) ret_ids in
          VariableDeclaration (typed_ids, None) in
            return_vars_dec :: obj.body in
      (* Check if the last statement is a Leave. If it's not,
         add the correct return. *)
      let body = match List.nth_opt body (List.length body - 1) with
      (* | Some (Expression (FunctionCall ("return", _))) ->  *)
      (*     body *)
      | None | Some Leave -> body
      | Some _ -> body @ [Leave] in
      let ctx = Ctx.set_fundec ctx ret_ids in
      let body = statements ctx body in
      aspf {|fn %s(%s) %s {
        %s
      }|} obj.name params ret_ty body
  | Leave -> 
      match ctx.fundec with
      | None -> failwith "Leave: we're not in a funcdec!"
      | Some ids ->
          let rets = match ids with
          | [] -> "()"
          | x::[] -> x
          | _ -> aspf "(%s)" @@ String.concat ", " ids in
          aspf "return Ok(%s)" rets
  
and statements ctx stmts =
  list (statement ctx) stmts ";\n" ^ ";"

let main stmts =
  let ctx = Ctx.make () in
  (* We must put the function switch call into a main function.*)
  let stmts = match stmts with
  | Block stmts -> stmts
  | _ -> failwith "unexpected ast structure" in
  let rec aggregate acc stmts =
    match stmts with
    | [] -> (acc, [])
    | FunctionDefinition _::_ -> (acc, stmts)
    | stmt::tl -> aggregate (stmt::acc) tl in
  let (stmts, funcdefs) = aggregate  [] stmts in
  let stmts = List.rev stmts in
  let maindef = FunctionDefinition {
    name = "entrypoint";
    params = None;
    returns = None;
    body = stmts
  } in
  let toplevel = maindef::funcdefs in
  let code = list (statement ctx) toplevel "\n" in
  aspf "%s\n%s" prelude code
