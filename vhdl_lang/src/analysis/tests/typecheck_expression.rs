// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) 2019, Olof Kraigher olof.kraigher@gmail.com

use super::*;

#[test]
fn test_integer_literal_expression_typecheck() {
    let mut builder = LibraryBuilder::new();
    let code = builder.in_declarative_region(
        "
constant good_a : integer := 1;
constant good_b : natural := 2;

constant bad_a : boolean := 3;

subtype my_bool is boolean;
constant bad_b : my_bool := 4;

        ",
    );

    let diagnostics = builder.analyze();
    check_diagnostics(
        diagnostics,
        vec![
            Diagnostic::error(
                code.s1("3"),
                "integer literal does not match type 'BOOLEAN'",
            ),
            Diagnostic::error(
                code.s1("4"),
                "integer literal does not match subtype 'my_bool'",
            ),
        ],
    );
}

#[test]
fn test_integer_literal_with_alias() {
    let mut builder = LibraryBuilder::new();
    let code = builder.in_declarative_region(
        "
alias alias_t is integer;
constant good : alias_t := 1;
constant bad : alias_t := false;
        ",
    );

    let diagnostics = builder.analyze();
    check_diagnostics(
        diagnostics,
        vec![Diagnostic::error(
            code.s1("false"),
            "'false' does not match alias 'alias_t'",
        )],
    );
}

#[test]
fn test_character_literal_expression() {
    let mut builder = LibraryBuilder::new();
    let code = builder.in_declarative_region(
        "
constant good : character := 'a';
constant bad : natural := 'b';
        ",
    );

    let diagnostics = builder.analyze();
    check_diagnostics(
        diagnostics,
        vec![Diagnostic::error(
            code.s1("'b'"),
            "character literal does not match subtype 'NATURAL'",
        )],
    );
}

#[test]
fn test_character_literal_expression_not_part_of_enum() {
    let mut builder = LibraryBuilder::new();
    let code = builder.in_declarative_region(
        "
type enum_t is ('a', 'b');
constant good : enum_t := 'a';
constant bad : enum_t := 'c';
        ",
    );

    let diagnostics = builder.analyze();
    check_diagnostics(
        diagnostics,
        vec![Diagnostic::error(
            code.s1("'c'"),
            "character literal does not match type 'enum_t'",
        )],
    );
}

#[test]
fn test_physical_literal_expression_typecheck() {
    let mut builder = LibraryBuilder::new();
    builder.in_declarative_region(
        "
constant good_a : time := ns;
constant good_b : time := 2 ps;
        ",
    );

    let diagnostics = builder.analyze();
    check_no_diagnostics(&diagnostics);
}

#[test]
fn test_string_literal_expression() {
    let mut builder = LibraryBuilder::new();
    let code = builder.in_declarative_region(
        "
signal good : string(1 to 3) := \"101\";
signal bad : natural := \"110\";
        ",
    );

    let diagnostics = builder.analyze();
    check_diagnostics(
        diagnostics,
        vec![Diagnostic::error(
            code.s1("\"110\""),
            "string literal does not match subtype 'NATURAL'",
        )],
    )
}

#[test]
fn test_string_literals_allowed_characters_for_array() {
    let mut builder = LibraryBuilder::new();
    let code = builder.in_declarative_region(
        "
type enum_t is ('a', foo);
type enum_vec_t is array (natural range <>) of enum_t;

signal good1 : bit_vector(1 to 1) := \"1\";
signal good2 : enum_vec_t(1 to 1) := \"a\";

signal bad2 : bit_vector(1 to 1) := \"2\";
signal bad3 : enum_vec_t(1 to 1) := \"b\";

type enum_vec2_t is array (natural range <>, natural range <>) of enum_t;
type enum_vec3_t is array (natural range <>) of enum_vec_t(1 to 1);
signal bad4 : enum_vec2_t(1 to 1, 1 to 1) := \"a\";
signal bad5 : enum_vec3_t(1 to 1) := \"a\";
        ",
    );

    let diagnostics = builder.analyze();
    check_diagnostics(
        diagnostics,
        vec![
            Diagnostic::error(code.s1("\"2\""), "type 'BIT' does not define character '2'"),
            Diagnostic::error(
                code.s1("\"b\""),
                "type 'enum_t' does not define character 'b'",
            ),
            Diagnostic::error(
                code.s("\"a\"", 2),
                "string literal does not match array type 'enum_vec2_t'",
            ),
            Diagnostic::error(
                code.s("\"a\"", 3),
                "string literal does not match array type 'enum_vec3_t'",
            ),
        ],
    )
}

#[test]
fn test_integer_selected_name_expression_typecheck() {
    let mut builder = LibraryBuilder::new();
    let code = builder.in_declarative_region(
        "
constant ival : integer := 999;

type rec_t is record
  elem : natural;
end record;

constant rval : rec_t := (elem => 0);

constant good_a : integer := ival;
constant good_b : natural := rval.elem;

constant bad_a : boolean := ival;

subtype my_bool is boolean;
constant bad_b : my_bool := rval.elem;

        ",
    );

    let diagnostics = builder.analyze();
    check_diagnostics(
        diagnostics,
        vec![
            Diagnostic::error(
                code.s("ival", 3),
                "constant 'ival' does not match type 'BOOLEAN'",
            ),
            Diagnostic::error(
                code.s("rval.elem", 2),
                "element declaration 'elem' does not match subtype 'my_bool'",
            ),
        ],
    );
}

#[test]
fn test_enum_literal_expression_typecheck() {
    let mut builder = LibraryBuilder::new();
    let code = builder.in_declarative_region(
        "
subtype my_bool is boolean;
constant good_a : boolean := true;
constant good_b : my_bool := false;

constant bad_a : integer := true;
constant bad_b : character := false;

        ",
    );

    let diagnostics = builder.analyze();
    check_diagnostics(
        diagnostics,
        vec![
            Diagnostic::error(
                code.s("true", 2),
                "'true' does not match integer type 'INTEGER'",
            ),
            Diagnostic::error(
                code.s("false", 2),
                "'false' does not match type 'CHARACTER'",
            ),
        ],
    );
}

#[test]
fn test_unique_function_typecheck() {
    let mut builder = LibraryBuilder::new();
    let code = builder.in_declarative_region(
        "
function fun1 return natural is
begin
    return 0;
end function;

function fun1 return boolean is
begin
    return 0;
end function;

constant good : integer := fun1;
constant bad : character := fun1;
        ",
    );

    let diagnostics = builder.analyze();
    check_diagnostics(
        diagnostics,
        vec![Diagnostic::error(
            code.s("fun1", 4),
            "'fun1' does not match type 'CHARACTER'",
        )],
    );
}

#[test]
fn test_unique_function_default_arg_typecheck() {
    let mut builder = LibraryBuilder::new();
    let code = builder.in_declarative_region(
        "
function fun1(arg : natural := 0) return natural is
begin
    return 0;
end function;

function fun1 return boolean is
begin
    return 0;
end function;

constant good : integer := fun1;
constant bad : character := fun1;
        ",
    );

    let diagnostics = builder.analyze();
    check_diagnostics(
        diagnostics,
        vec![Diagnostic::error(
            code.s("fun1", 4),
            "'fun1' does not match type 'CHARACTER'",
        )],
    );
}

#[test]
fn test_ambiguous_function_default_arg_typecheck() {
    let mut builder = LibraryBuilder::new();
    let code = builder.in_declarative_region(
        "
function fun1(arg : natural := 0) return natural is
begin
    return 0;
end function;

function fun1(arg : boolean := false) return natural is
begin
    return 0;
end function;

constant bad: integer := fun1;
        ",
    );

    let diagnostics = builder.analyze();
    check_diagnostics(
        diagnostics,
        vec![
            Diagnostic::error(code.s("fun1", 3), "ambiguous use of 'fun1'")
                .related(code.s("fun1", 1), "migth be fun1[NATURAL return NATURAL]")
                .related(code.s("fun1", 2), "migth be fun1[BOOLEAN return NATURAL]"),
        ],
    );
}

#[test]
fn test_name_can_be_indexed() {
    let mut builder = LibraryBuilder::new();
    let code = builder.in_declarative_region(
        "
constant foo : natural := 0;
constant bar : natural := foo(0);
        ",
    );

    let diagnostics = builder.analyze();
    check_diagnostics(
        diagnostics,
        vec![Diagnostic::error(
            code.s("foo", 2),
            "subtype 'NATURAL' cannot be indexed",
        )],
    );
}

#[test]
fn test_name_can_be_sliced() {
    let mut builder = LibraryBuilder::new();
    let code = builder.in_declarative_region(
        "
constant foo : natural := 0;
constant bar : natural := foo(0 to 0);
        ",
    );

    let diagnostics = builder.analyze();
    check_diagnostics(
        diagnostics,
        vec![Diagnostic::error(
            code.s("foo", 2),
            "subtype 'NATURAL' cannot be sliced",
        )],
    );
}

#[test]
fn test_access_type_can_be_indexed() {
    let mut builder = LibraryBuilder::new();
    builder.in_declarative_region(
        "
type arr_t is array (0 to 1) of natural;
type access_t is access arr_t;

procedure proc is
   variable myvar : access_t;
   variable foo : natural;
begin
    foo := myvar(0);
end procedure;
        ",
    );

    let diagnostics = builder.analyze();
    check_no_diagnostics(&diagnostics);
}

#[test]
fn test_type_conversion() {
    let mut builder = LibraryBuilder::new();
    builder.in_declarative_region(
        "
  constant foo : natural := natural(0);
        ",
    );

    let diagnostics = builder.analyze();
    check_no_diagnostics(&diagnostics);
}

#[test]
fn test_indexed_array_dimension_check() {
    let mut builder = LibraryBuilder::new();
    let code = builder.in_declarative_region(
        "
type arr1_t is array (0 to 1) of natural;
type arr2_t is array (0 to 1, 0 to 1) of natural;
constant foo1 : arr1_t := (0, 1);
constant foo2 : arr2_t := ((0, 1), (2, 3));

constant bar1 : natural := foo1(0);
constant bar2 : natural := foo1(0, 1);
constant bar3 : natural := foo2(0);
constant bar4 : natural := foo2(0, 1);
        ",
    );

    let diagnostics = builder.analyze();
    check_diagnostics(
        diagnostics,
        vec![
            Diagnostic::error(
                code.s("foo1(0, 1)", 1),
                "Number of indexes does not match array dimension",
            )
            .related(
                code.s("arr1_t", 1),
                "Array type 'arr1_t' has 1 dimension, got 2 indexes",
            ),
            Diagnostic::error(
                code.s("foo2(0)", 1),
                "Number of indexes does not match array dimension",
            )
            .related(
                code.s("arr2_t", 1),
                "Array type 'arr2_t' has 2 dimensions, got 1 index",
            ),
        ],
    );
}

#[test]
fn test_disambiguates_indexed_name_and_function_call() {
    let mut builder = LibraryBuilder::new();
    let code = builder.in_declarative_region(
        "
constant foo : natural := 0;
constant bar : natural := foo(arg => 0);
        ",
    );

    let diagnostics = builder.analyze();
    check_diagnostics(
        diagnostics,
        vec![Diagnostic::error(
            code.s("foo", 2),
            "constant 'foo' cannot be the prefix of a function call",
        )],
    );
}

#[test]
fn test_typechecks_expression_for_type_mark_with_subtype_attribute() {
    let mut builder = LibraryBuilder::new();
    let code = builder.in_declarative_region(
        "
signal sig0 : integer_vector(0 to 7);
signal sig1 : sig0'subtype := false;
signal sig2 : sig0'subtype := (others => 0); -- ok
",
    );
    let diagnostics = builder.analyze();
    check_diagnostics(
        diagnostics,
        vec![Diagnostic::error(
            code.s1("false"),
            "'false' does not match array type 'INTEGER_VECTOR'",
        )],
    );
}

#[test]
fn qualified_expression_type_mark() {
    let mut builder = LibraryBuilder::new();
    let code = builder.in_declarative_region(
        "
signal good : natural := integer'(0);
signal bad1 : natural := integer'(\"hello\");
signal bad2 : natural := string'(\"hello\");
",
    );
    let diagnostics = builder.analyze();
    check_diagnostics(
        diagnostics,
        vec![
            Diagnostic::error(
                code.s1("(\"hello\")"),
                "string literal does not match integer type 'INTEGER'",
            ),
            Diagnostic::error(
                code.s1("string'(\"hello\")"),
                "array type 'STRING' does not match subtype 'NATURAL'",
            ),
        ],
    );
}