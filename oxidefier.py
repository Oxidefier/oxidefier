from collections import defaultdict
import json
from pathlib import Path
import re
import sys
from typing import Union


# Indent each line of the block, except empty lines
def indent(block: str) -> str:
    indentation = "    "
    return "\n".join(
        line if line == "" else indentation + line
        for line in block.split("\n")
    )


def paren(condition: bool, value: str) -> str:
    return f"({value})" if condition else value


def name_to_rust(name: str) -> str:
    reserved_names = [
        "end",
        "mod",
        "return",
    ]

    if name in reserved_names:
        return name + "_"

    # We do this replacement to get the name accepted in Coq while still keeping a code
    # which is readable.
    return name.replace("usr$", "ᵤ").replace("$", "ₓ")


def names_to_rust(names: list[str]) -> str:
    if len(names) == 0:
        return "()"

    if len(names) == 1:
        return name_to_rust(names[0])

    return "(" + ', '.join(name_to_rust(name) for name in names) + ")"


def variable_name_to_name(variable_name) -> str:
    return variable_name.get('name')


def variable_name_to_rust(variable_name) -> str:
    return name_to_rust(variable_name_to_name(variable_name))


def variable_names_to_rust(variable_names: list) -> str:
    return names_to_rust(
        [variable_name_to_name(variable_name) for variable_name in variable_names]
    )


def block_to_rust(
    return_variables: Union[None, list],
    node,
) -> tuple[str, set[str]]:
    node_type = node.get('nodeType')

    if node_type == 'YulBlock':
        nodes = node.get('statements', [])
        statements: list[str] = []
        mutated_variables: set[str] = set()

        for node in reversed(nodes):
            # We ignore the functions here as they are handled separately
            if node.get('nodeType') == 'YulFunctionDefinition':
                continue

            statement, new_mutated_variables = \
                statement_to_rust(node, return_variables, mutated_variables)
            statements += [statement]
            mutated_variables.update(new_mutated_variables)

        return "\n".join(reversed(statements)), mutated_variables

    return f"// Unsupported block node type: {node_type}", set()


def is_pre_empty_block(node) -> bool:
    return node.get('nodeType') == 'YulBlock' and len(node.get('statements', [])) == 0


# Returns the translation of the statement and the list of variables that are
# mutated by the statement.
def statement_to_rust(
    node,
    return_variables: Union[None, list],
    next_mutated_variables: set[str],
) -> tuple[str, set[str]]:
    node_type = node.get('nodeType')

    if node_type == 'YulBlock':
        return block_to_rust(return_variables, node)

    elif node_type == 'YulFunctionDefinition':
        # We ignore this case because we only handle top-level function definitions
        return "// Function definition not expected at block level", set()

    elif node_type == 'YulVariableDeclaration':
        variable_names = node.get('variables', [])
        variables = variable_names_to_rust(variable_names)
        value = \
            expression_to_rust(node.get('value')) \
            if node.get('value') is not None \
            else "U256::ZERO"
        mut_flag = \
            " mut" \
            if len(variable_names) == 1 and variable_name_to_rust(variable_names[0]) in next_mutated_variables \
            else ""
        return f"let{mut_flag} {variables} = {value};", set()

    elif node_type == 'YulAssignment':
        variable_names = node.get('variableNames', [])
        variables = variable_names_to_rust(variable_names)
        value = expression_to_rust(node.get('value'))
        return (
            f"{variables} = {value};",
            set(variable_name_to_name(variable_name) for variable_name in variable_names)
        )

    elif node_type == 'YulExpressionStatement':
        return expression_to_rust(node.get('expression')) + ";", set()

    elif node_type == 'YulIf':
        condition = expression_to_rust(node.get('condition'))
        then_body, mutated_variables = block_to_rust(return_variables, node.get('body'))
        return (
            "if " + condition + " != U256::ZERO {\n" + \
            indent(then_body) + "\n" + \
            "}",
            mutated_variables
        )

    elif node_type == 'YulSwitch':
        expression = expression_to_rust(node.get('expression'))
        cases = [
            (
                expression_to_rust(case.get('value')),
                block_to_rust(return_variables, case.get('body')),
            )
            for case in node.get('cases', [])
            # TODO: handle the default case in a switch
            if case.get('value') != "default"
        ]
        mutated_variables = set()
        for _, (_, mutated_variables) in cases:
            mutated_variables.update(mutated_variables)


        return (
            "// switch\n" + \
            f"let δ = {expression};\n" + \
            "\n} else ".join(
                f"if δ == {value} {{\n" +
                indent(body)
                for value, (body, _) in cases
            ) + "\n" + \
            "}",
            mutated_variables
        )

    elif node_type == 'YulBreak':
        return "break;", set()

    elif node_type == 'YulContinue':
        return "continue;", set()

    elif node_type == 'YulLeave':
        if return_variables is None:
            return "return;", set()
        return "return Ok(" + variable_names_to_rust(return_variables) + ");", set()

    elif node_type == 'YulForLoop':
        if not is_pre_empty_block(node.get('pre')):
            return statement_to_rust({
                'nodeType': 'YulBlock',
                'statements':
                    node.get('pre').get('statements', []) + [{
                        'nodeType': 'YulForLoop',
                        'pre': {
                            'nodeType': 'YulBlock',
                            'statements': [],
                        },
                        'condition': node.get('condition'),
                        'post': node.get('post'),
                        'body': node.get('body'),
                    }]
            }, return_variables, next_mutated_variables)

        condition = expression_to_rust(node.get('condition'))
        post, post_mutated_variables = block_to_rust(return_variables, node.get('post'))
        body, body_mutated_variables = block_to_rust(return_variables, node.get('body'))

        return (
            "// for loop\n" + \
            "while " + condition + " != U256::ZERO {\n" + \
            indent(
                "// body\n" + \
                "{\n" + \
                indent(body) + "\n" + \
                "}\n" + \
                "// post\n" + \
                "{\n" + \
                indent(post) + "\n" + \
                "}"
            ) + "\n" + \
            "}",
            body_mutated_variables.union(post_mutated_variables)
        )

    return f"// Unsupported statement node type: {node_type}", set()


def number_to_u256(number: str) -> str:
    if number.startswith('0x'):
        int_number = int(number, 16)  # Hex string (e.g., "0x1a" → 26)
    else:
        int_number = int(number)

    if int_number < 2**128:
        # Fits in u128, use U256::from with hex notation
        return f"U256::from(0x{int_number:x}u128)"
    else:
        # Requires byte array conversion (32 bytes, big-endian)
        hex_bytes = int_number.to_bytes(32, byteorder='big')
        hex_str = ', '.join(f'0x{b:02x}' for b in hex_bytes)
        return f"U256::from_be_slice(&[{hex_str}])"


def expression_to_rust(node) -> str:
    node_type = node.get('nodeType')

    if node_type == 'YulFunctionCall':
        func_name = variable_name_to_rust(node['functionName'])
        args: list[str] = [expression_to_rust(arg) for arg in node.get('arguments', [])]
        return func_name + "(" + ", ".join(args + ["context"]) + ")?"

    if node_type == 'YulIdentifier':
        return variable_name_to_rust(node)

    if node_type == 'YulLiteral':
        if node['kind'] == 'string':
            return "from_hex(\"" + node['hexValue'].ljust(64, '0') + "\")"
        return number_to_u256(node.get('value'))

    return f"// Unsupported expression node type: {node_type}"


def function_result_type(arity: int) -> str:
    if arity == 0:
        return "()"
    elif arity == 1:
        return "U256"

    return "(" + ", ".join(["U256"] * arity) + ")"


def function_definition_to_rust(node) -> str:
    name = variable_name_to_rust(node)
    returnVariables = node.get('returnVariables', [])
    body, mutated_variables = block_to_rust(returnVariables, node.get('body'))
    param_names: list[str] = [
        variable_name_to_rust(param)
        for param in node.get('parameters', [])
    ]
    params = ', '.join(
        [
            ("mut " if name in mutated_variables else "") +
            name + ": U256"
            for name in param_names
        ] +
        ["context: &mut Context<CI>"]
    )

    return \
        f"pub fn {name}<CI>({params}) -> YulOutput<" + \
        function_result_type(len(returnVariables)) + ">\n" + \
        "where\n" + \
        indent("Context<CI>: ContractInteractions,\n") + \
        "{\n" + \
        indent(
            "".join(
                "let " +
                ("mut " if variable_name_to_name(returnVariable) in mutated_variables else "") +
                variable_name_to_rust(returnVariable) + " = U256::ZERO;\n"
                for returnVariable in returnVariables
            ) + \
            body + "\n" + \
            "Ok(" + variable_names_to_rust(returnVariables) + ")"
        ) + "\n" + \
        "}"


# Get the names of the functions called in a function.
# We take care of sorting the names in alphabetical order so that the output is
# deterministic.
def get_function_dependencies(function_node) -> list[str]:
    dependencies = set()

    def traverse(node):
        if isinstance(node, dict):
            if node.get('nodeType') == 'YulFunctionCall':
                function_name = node['functionName']['name']
                dependencies.add(function_name)
            for key in sorted(node.keys()):
                traverse(node[key])
        elif isinstance(node, list):
            for item in node:
                traverse(item)

    # Start traversal from the 'statements' field
    traverse(function_node.get('body', {}))

    return sorted(dependencies)


def topological_sort(functions: dict[str, list[str]]) -> list[str]:
    # Create a graph representation
    graph = defaultdict(list)
    all_functions = set()
    for function, called_functions in sorted(functions.items()):
        all_functions.add(function)
        for called in called_functions:
            graph[function].append(called)
            all_functions.add(called)

    # Helper function for DFS
    def dfs(node, visited, stack, path):
        visited.add(node)
        path.add(node)

        for neighbor in graph[node]:
            if neighbor in path:
                cycle = list(path)[list(path).index(neighbor):] + [neighbor]
                print(f"Warning: Cycle detected: {' -> '.join(cycle)}")
            elif neighbor not in visited:
                dfs(neighbor, visited, stack, path)

        path.remove(node)
        stack.append(node)

    visited = set()
    stack = []

    # Perform DFS for each unvisited node
    for function in sorted(all_functions):
        if function not in visited:
            dfs(function, visited, stack, set())

    return stack


def order_functions(ordered_names: list[str], function_nodes: list) -> list:
    # Create a dictionary for quick lookup of index in ordered_names
    name_order: dict[str, int] = {
        name: index
        for index, name in enumerate(ordered_names)
    }

    # Define a key function that returns the index of the function name in ordered_names
    def key_func(node):
        return name_order.get(node.get('name'), len(ordered_names))

    # Sort the function_nodes using the key function
    return sorted(function_nodes, key=key_func)


def top_level_to_rust(node) -> str:
    node_type = node.get('nodeType')

    if node_type == 'YulBlock':
        functions_dependencies: dict[str, list[str]] = {}
        for statement in node.get('statements', []):
            if statement.get('nodeType') == 'YulFunctionDefinition':
                function_name = statement.get('name')
                dependencies = get_function_dependencies(statement)
                functions_dependencies[function_name] = dependencies
        ordered_function_names = topological_sort(functions_dependencies)
        ordered_functions = \
            order_functions(ordered_function_names, node.get('statements', []))
        functions = [
            function_definition_to_rust(function)
            for function in ordered_functions
            if function.get('nodeType') == 'YulFunctionDefinition'
        ]
        body = \
            "pub fn body<CI: ContractInteractions>(context: &mut Context<CI>) -> YulOutput<()>\n" + \
            "where\n" + \
            indent("Context<CI>: ContractInteractions,\n") + \
            "{\n" + \
            indent(
                block_to_rust(None, node)[0] + "\n" + \
                "Ok(())"
            ) + "\n" + \
            "}"
        return ("\n\n").join(functions + [body])

    return f"// Unsupported top-level node type: {node_type}"


def object_to_rust(node) -> str:
    node_type = node.get('nodeType')

    if node_type == 'YulObject':
        # The names end with a generated number, we remove it
        name = re.sub(r'_[0-9]+$', '', node['name']).lower()
        name = re.sub(r'_[0-9]+_deployed$', '_deployed', name)
        return \
            "pub mod " + name + " {\n" + \
            indent(
                "use alloy_primitives::U256;" + "\n" + \
                "use evm_opcodes::*;" + "\n" + \
                "\n" + \
                object_to_rust(node['code']) + "\n" + \
                "".join(
                    "\n" +
                    object_to_rust(child) + "\n"
                    for child in node.get('subObjects', [])
                    if child.get('nodeType') != 'YulData'
                )
            ) + \
            "}"

    elif node_type == 'YulCode':
        return top_level_to_rust(node['block'])

    elif node_type == 'YulData':
        return "// Data object not expected"

    return f"// Unsupported object node type: {node_type}"


# Return if the file was not empty
def file_to_rust(contract_name: str, file_path: Path):
    with open(file_path, 'r') as file:
        data = json.load(file)

    if data is None:
        return False

    rust_code = object_to_rust(data)
    first_object_name = data['name'].lower()

    rust_file = """// Generated by Oxidefier

#![allow(mixed_script_confusables)]
#![allow(non_snake_case)]
#![allow(uncommon_codepoints)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

use alloy_primitives::U256;
use evm_opcodes::*;

"""
    rust_file += rust_code
    rust_file += """

fn main() {
    let mut context = Context {
        contract_interactions: std::marker::PhantomData::<DummyContractInteractions>,
        memory: Memory::new(),
        immutables: std::collections::HashMap::new(),
        address: U256::from(123),
        caller: U256::from(124),
        callvalue: U256::from(12),
        gas: U256::from(100 * 1000),
        timestamp: U256::from(1000 * 1000),
        calldata: vec![],
        chain_id: U256::from(123456),
    };

    let result = morpho::morpho_deployed::fun_withdraw(
        U256::from(0),
        U256::from(1),
        U256::from(2),
        U256::from(3),
        U256::from(4),
        &mut context,
    );
    println!("result: {:#?}", result);
    println!("context: {:#?}", context);
}
"""

    output_path = Path("output") / contract_name
    Path(output_path).mkdir(parents=True, exist_ok=True)
    # Create an "src/" folder in the output folder
    src_folder = output_path / "src"
    src_folder.mkdir(parents=True, exist_ok=True)
    main_rs_file = src_folder / Path("main.rs")
    main_rs_file.write_text(rust_file)


def main():
    """python oxidefier.py <path_to_yul_json_file> <contract_name>"""
    file_path = sys.argv[1]
    contract_name = sys.argv[2]

    file_to_rust(contract_name, Path(file_path))

    cargo_toml = f"""[package]
name = "{contract_name}"
version = "0.1.0"
edition.workspace = true

[dependencies]
alloy-primitives.workspace = true
evm_opcodes.workspace = true
"""
    cargo_toml_file = Path("output") / contract_name / "Cargo.toml"
    cargo_toml_file.write_text(cargo_toml)


if __name__ == "__main__":
    main()
