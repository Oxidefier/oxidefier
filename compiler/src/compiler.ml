let test_ast = 
  let open Ast in
  let zero_lit = NumberLiteral ("0", None) in
  let zero = Literal zero_lit in
  let one = Literal (NumberLiteral ("1", None)) in
  let vardec = 
    VariableDeclaration ([("a", None)], Some zero) in
  let assign =
    Assignment (["a"], one) in
  let funcall = Expression (FunctionCall ("f", [zero; one])) in
  Block [vardec; assign; funcall]

let () =
  if Array.length Sys.argv < 2 then (
    prerr_endline "Usage: yul_parser <file.json>";
    exit 1
  );
  let filename = Sys.argv.(1) in
  let _ast = Parser.load_yul_ast filename in
  print_endline @@ Ast.show_statement _ast;
  print_endline "Successfully parsed Yul JSON into AST.";
  print_endline @@ Rust.statement test_ast

