open Ast
open Utils

let get_id =
  let id = ref 0 in
  fun () ->
  incr id;
  "tmp_" ^ string_of_int !id

type ret = {
  fn_id: string;
  tmp_id: string;
  args: expression list;
}

module List = struct
  include List
  let rfold_left l acc f = fold_left f acc l
end

let rec flatten_funccall id exprl =
  (* Collect what must change *)
  let shifts = List.rfold_left exprl [] (fun acc expr ->
    match expr with
        | FunctionCall (id, exprl) ->
            let tmp_id = get_id () in
            let ret = {
              fn_id = id;
              tmp_id = tmp_id;
              args = exprl } in
            ret::acc
        | _ -> acc)  in
  (* forge a new variable declaration *)
  let stmts = List.rmap shifts (fun ret ->
  (* Recurse on the previously made shift *)
    let funccall, stmts = flatten_funccall ret.fn_id ret.args in
    stmts @
    [VariableDeclaration (
      ([ret.tmp_id, None]),
          Some (funccall))]) |> List.flatten in
  (* replace the arg with the variable *)
  let args = List.rmap exprl (function
    | FunctionCall (id, _) ->
        begin match List.find_opt (fun ret -> ret.fn_id == id) shifts with
        | None -> failwith "unexpected"
        | Some ret -> Identifier ret.tmp_id end
    | oth -> oth) in
  FunctionCall (id, args), stmts


  let flatten_expr = function
    | FunctionCall (id, exprl) ->
        flatten_funccall id exprl
    | oth -> oth,[]

  let rec list stmts =
    List.rmap stmts flatten_statement |> List.flatten

  and flatten_statement = function
    | Block stmts ->
        let stmts = list stmts in
        [Block stmts]
    | FunctionDefinition funcdef ->
        let body = list funcdef.body in
        [FunctionDefinition {funcdef with body = body}]
    | VariableDeclaration (tyl, expr) ->
        begin match expr with
      | None -> [VariableDeclaration (tyl, None)]
      | Some expr ->
          let expr, stmts = flatten_expr expr in
          stmts @ [VariableDeclaration (tyl, Some expr)]
        end
      | Expression expr ->
          let expr, stmts = flatten_expr expr in
          stmts @ [Expression expr]
      | oth -> [oth]

  let flatten_calls stmt =
    Block (flatten_statement stmt)
