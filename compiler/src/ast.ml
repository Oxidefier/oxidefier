type identifier = string [@@deriving show]
type type_name = string [@@deriving show]

type literal =
  | NumberLiteral of string * type_name option
  | StringLiteral of string * type_name option
  | TrueLiteral of type_name option
  | FalseLiteral of type_name option
  [@@deriving show]

type expression =
  | FunctionCall of identifier * expression list
  | Identifier of identifier
  | Literal of literal
  [@@deriving show]

type typed_identifier = identifier * type_name option [@@deriving show]
type typed_identifier_list = typed_identifier list [@@deriving show]
type identifier_list = identifier list [@@deriving show]

type statement =
  | Block of statement list
  | FunctionDefinition of {
      name: identifier;
      params: typed_identifier_list option;
      returns: typed_identifier_list option;
      body: statement list;
    }
  | VariableDeclaration of typed_identifier_list * expression option
  | Assignment of identifier_list * expression
  | If of expression * statement list
  | Expression of expression
  | Switch of expression * (literal * statement list) list * (statement list option)
  | ForLoop of statement list * expression * statement list * statement list
  | Break
  | Continue
  | Leave

  [@@deriving show]


