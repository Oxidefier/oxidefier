open Ast
open Yojson.Basic.Util

let rec expression_of_yul_json json =
  match json |> member "nodeType" |> to_string with
  | "YulFunctionCall" ->
      let name = json |> member "functionName" |> member "name" |> to_string in
      let args = json |> member "arguments" |> to_list |> List.map expression_of_yul_json in
      FunctionCall (name, args)
  | "YulIdentifier" ->
      Identifier (json |> member "name" |> to_string)
  | "YulLiteral" ->
      let value = json |> member "value" |> to_string in
      let kind = json |> member "kind" |> to_string in
      let lit =
        match kind with
        | "number" -> NumberLiteral (value, None)
        | "string" -> StringLiteral (value, None)
        | "bool" ->
            if value = "true" then TrueLiteral None else FalseLiteral None
        | _ -> failwith ("Unknown literal kind: " ^ kind)
      in
      Literal lit
  | other -> failwith ("Unknown expression nodeType: " ^ other)

(* === Statement Parser === *)

let rec statement_of_yul_json json =
  match json |> member "nodeType" |> to_string with
  | "YulBlock" ->
      let stmts = json |> member "statements" |> to_list |> List.map statement_of_yul_json in
      Block stmts
  | "YulExpressionStatement" ->
      let expr = json |> member "expression" in
      Expression (expression_of_yul_json expr)
  | "YulVariableDeclaration" ->
      let vars =
        json |> member "variables" |> to_list |> List.map (fun v ->
          (v |> member "name" |> to_string, None)
        )
      in
      let value = json |> member "value" |> to_option expression_of_yul_json in
      VariableDeclaration (vars, value)
  | "YulAssignment" ->
      let vars =
        json |> member "variableNames" |> to_list |> List.map (fun v ->
          v |> member "name" |> to_string
        )
      in
      let expr = json |> member "value" |> expression_of_yul_json in
      Assignment (vars, expr)
  | "YulIf" ->
      let cond = json |> member "condition" |> expression_of_yul_json in
      let body = json |> member "body" |> statement_of_yul_json in
      If (cond, match body with Block stmts -> stmts | stmt -> [stmt])
  | "YulFunctionDefinition" ->
      let name = json |> member "name" |> to_string in
      let params =
        json |> member "parameters" |> to_option (fun l ->
          l |> to_list |> List.map (fun p ->
            (p |> member "name" |> to_string, None)
          )
        )
      in
      let returns =
        json |> member "returnVariables" |> to_option (fun l ->
          l |> to_list |> List.map (fun r ->
            (r |> member "name" |> to_string, None)
          )
        )
      in
      let body = json |> member "body" |> statement_of_yul_json in
      FunctionDefinition {
        name;
        params;
        returns;
        body = match body with Block stmts -> stmts | stmt -> [stmt]
      }
  | "YulSwitch" ->
      let expr = json |> member "expression" in
      let expr =
        match expr |> member "nodeType" |> to_string with
        | "YulIdentifier" -> Identifier (expr |> member "name" |> to_string)
        | _ -> expression_of_yul_json expr
      in
      let cases =
        json |> member "cases" |> to_list |> List.fold_left (fun (cs, def) case ->
          match case |> member "nodeType" |> to_string with
          | "YulCase" ->
              let val_node = case |> member "value" in
              if val_node = `String "default" then
                (cs, Some (case |> member "body" |> member "statements" |> to_list |> List.map statement_of_yul_json))
              else
                let lit = expression_of_yul_json val_node in
                let lit = match lit with Literal l -> l | _ -> failwith "Expected literal in case" in
                let body = case |> member "body" |> member "statements" |> to_list |> List.map statement_of_yul_json in
                ((lit, body) :: cs, def)
          | _ -> (cs, def)
        ) ([], None)
      in
      let cases, default = cases in
      Switch (expr, List.rev cases, default)
  | "YulForLoop" ->
      let pre = json |> member "pre" |> statement_of_yul_json in
      let cond = json |> member "condition" |> expression_of_yul_json in
      let post = json |> member "post" |> statement_of_yul_json in
      let body = json |> member "body" |> statement_of_yul_json in
      let get_block stmts = match stmts with Block b -> b | s -> [s] in
      ForLoop (get_block pre, cond, get_block post, get_block body)
  | "YulBreak" -> Break
  | "YulContinue" -> Continue
  | "YulLeave" -> Leave
  | other -> failwith ("Unknown statement nodeType: " ^ other)

(* === Entry Point === *)

let load_yul_ast (filename : string) : statement =
  let json = Yojson.Basic.from_file filename in
  let code_block = json |> member "subObjects" 
  |> Yojson.Basic.Util.to_list
  |> List.hd
  |> member "code" |> member "block" in
  statement_of_yul_json code_block


