from collections import defaultdict
import json
from pathlib import Path
import sys
from typing import Callable, Union


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
) -> str:
    node_type = node.get('nodeType')

    if node_type == 'YulBlock':
        nodes = node.get('statements', [])
        statements: list[str] = []

        for node in nodes:
            # We ignore the functions here as they are handled separately
            if node.get('nodeType') == 'YulFunctionDefinition':
                continue

            statement = statement_to_rust(node)
            statements += [statement]

        if return_variables is not None:
            suffix = [variable_names_to_rust(return_variables)]
        else:
            suffix = []

        return "\n".join(statements + suffix)

    return f"// Unsupported block node type: {node_type}"


def is_pre_empty_block(node) -> bool:
    return node.get('nodeType') == 'YulBlock' and len(node.get('statements', [])) == 0


def statement_to_rust(node) -> str:
    node_type = node.get('nodeType')

    if node_type == 'YulBlock':
        return block_to_rust(None, node)

    elif node_type == 'YulFunctionDefinition':
        # We ignore this case because we only handle top-level function definitions
        return "// Function definition not expected at block level"

    elif node_type == 'YulVariableDeclaration':
        variable_names = node.get('variables', [])
        variables = variable_names_to_rust(variable_names)
        value = \
            expression_to_rust(node.get('value')) \
            if node.get('value') is not None \
            else "U256::from(0)"
        mut_flag = " mut" if len(variable_names) == 1 else ""
        return f"let{mut_flag} {variables} = {value};"

    elif node_type == 'YulAssignment':
        variable_names = node.get('variableNames', [])
        variables = variable_names_to_rust(variable_names)
        value = expression_to_rust(node.get('value'))
        return f"{variables} = {value};"

    elif node_type == 'YulExpressionStatement':
        return expression_to_rust(node.get('expression')) + ";"

    elif node_type == 'YulIf':
        condition = expression_to_rust(node.get('condition'))
        then_body = block_to_rust(None, node.get('body'))
        return \
            "if " + condition + " != U256::from(0) {\n" + \
            indent(then_body) + "\n" + \
            "}"

    elif node_type == 'YulSwitch':
        expression = expression_to_rust(node.get('expression'))
        cases = [
            (
                expression_to_rust(case.get('value')),
                block_to_rust(None, case.get('body')),
            )
            for case in node.get('cases', [])
            # TODO: handle the default case in a switch
            if case.get('value') != "default"
        ]
        return \
            "// switch\n" + \
            f"let δ = {expression};\n" + \
            "\n} else ".join(
                f"if δ == {value} {{\n" +
                indent(body)
                for value, body in cases
            ) + "\n" + \
            "}"

    elif node_type == 'YulBreak':
        return "break;"

    elif node_type == 'YulContinue':
        return "continue;"

    elif node_type == 'YulLeave':
        return "return;"

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
            })

        condition = expression_to_rust(node.get('condition'))
        post = block_to_rust(None, node.get('post'))
        body = block_to_rust(None, node.get('body'))

        return (
            "// for loop\n" + \
            "while " + condition + " != U256::from(0) {\n" + \
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
            "}"
        )

    return f"// Unsupported statement node type: {node_type}"


def number_to_u256(number: str) -> str:
    if number.startswith('0x'):
        number = int(number, 16)  # Hex string (e.g., "0x1a" → 26)
    else:
        number = int(number)

    if number < 2**128:
        # Fits in u128, use U256::from with hex notation
        return f"U256::from(0x{number:x}u128)"
    else:
        # Requires byte array conversion (32 bytes, big-endian)
        hex_bytes = number.to_bytes(32, byteorder='big')
        hex_str = ', '.join(f'0x{b:02x}' for b in hex_bytes)
        return f"U256::from_be_slice(&[{hex_str}])"


def expression_to_rust(node) -> str:
    node_type = node.get('nodeType')

    if node_type == 'YulFunctionCall':
        func_name = variable_name_to_rust(node['functionName'])
        args: list[str] = [expression_to_rust(arg) for arg in node.get('arguments', [])]
        return func_name + "(" + ", ".join(args + ["memory"]) + ")?"

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
    param_names: list[str] = [
        variable_name_to_rust(param)
        for param in node.get('parameters', [])
    ]
    params = ', '.join(
        ["mut " + name + ": U256" for name in param_names] +
        ["memory: &mut Memory"]
    )
    body = block_to_rust(None, node.get('body'))
    returnVariables = node.get('returnVariables', [])
    return \
        f"pub fn {name}({params}) -> YulOutput<" + \
        function_result_type(len(returnVariables)) + "> {\n" + \
        indent(
            "".join(
                "let mut " + variable_name_to_rust(returnVariable) + " = U256::ZERO;\n"
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
            "pub fn body(memory: &mut Memory) -> YulOutput<()> {\n" + \
            indent(
                block_to_rust(None, node) + "\n" + \
                "Ok(())"
            ) + "\n" + \
            "}"
        return ("\n\n").join(functions + [body])

    return f"// Unsupported top-level node type: {node_type}"


def object_to_rust(node) -> str:
    node_type = node.get('nodeType')

    if node_type == 'YulObject':
        return \
            "pub mod " + node['name'].lower() + " {\n" + \
            indent(
                "use alloy_primitives::U256;" + "\n" + \
                "use crate::opcode::*;" + "\n" + \
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


def main():
    """Input: JSON file with Yul AST"""
    with open(sys.argv[1], 'r') as file:
        data = json.load(file)

    rust_code = object_to_rust(data)

    print("// Generated by Oxidefier")
    print("mod i256;")
    print("#[macro_use]")
    print("mod macros;")
    print("mod opcode;")
    print()
    print(rust_code)
    print()
    print("""use alloy_primitives::U256;

fn main() {
    let mut memory = opcode::Memory::new();
    let result = plonkverifier_482::plonkverifier_482_deployed::fun_Verify(U256::ZERO, U256::ZERO, &mut memory);
    println!("result: {:#?}", result);
    println!("memory: {:#?}", memory);
}
""")


if __name__ == "__main__":
    main()
