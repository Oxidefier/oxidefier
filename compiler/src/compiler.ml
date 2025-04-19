let () =
  if Array.length Sys.argv < 2 then (
    prerr_endline "Usage: yul_parser <file.json>";
    exit 1
  );
  let filename = Sys.argv.(1) in
  let _ast = Parser.load_yul_ast filename in
  print_endline @@ Ast.show_statement _ast;
  print_endline "Successfully parsed Yul JSON into AST."

