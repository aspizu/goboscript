// Expressions:

    // Operators:
    
        // Unary:
        - 1         // Minus
        ! 1         // Not
        
        // Binary:
        1 + 2       // Add
        1 - 2       // Sub
        1 * 2       // Mul
        1 / 2       // Div
        1 & 2       // And
        1 | 2       // Or
        1 % 2       // Mod
        1 = 2       // Equal To
        1 < 2       // Less Than
        1 > 2       // Greater Than
        "a" ^ "b"   // Join

    // Literals:

        // Numbers:
        1024            // Integer, same as "1024"
        1024.5126       // Decimal, same as "1024.5126"

        // Strings:
        "Name\"\\"      // Only escape sequences are \" and \\

        // Booleans:
        true            // Same as "1"
        false           // Same as "0"

    // Blocks:
        
        // Statement Blocks:
        blockname expression1, expression2, expression3;
        blockname;

        // Reporter Blocks:
        blockname(expreesion1, expression2, expression3)
        blockname()

    // Variables:
        any_name
    
        variable_name = expression;         // Declare+Assign local variable

    // Lists:
        any_name[expression]

        #any_name   // Length of list
    
        list = [expression1, expression2,  expression3];    // Declare+Assign local list
        list = []; // declare empty list

        list = []; // delete all of list

        list[expression] = expression; // replace element expression of list with expression

            somelist.delete  10;
            somelist.insert  10, item;
            somelist.append  item;
            say  somelist.contains(item);
            say  somelist.index(item);
        
    // Control

    if boolean_expression {
        statementlist...
    }

    if boolean_expression {
        statementlist...
    } else {
        statementlist...
    }

    repeat expression {
        statementlist...
    }

    forever {
        statementlist...
    }

    until {
        statementlist...
    }

    // Hat Blocks:

        whenflagclicked {
            statement1;
            statement2;
            statement3;
        }

    // Procedure Definitions:

        // run-without-screen-refresh is opt-out using nowarp
        nowarp def procedure_name(arg1, arg2, arg3) {
            statement1;
            statement2 arg1; // arg1 here becomes a argument block
            procedure_name 1, 2, 3;
        }

        // Proecdures can only be defined top-level and are local to this file

    // Pre processor and sprite declarations

    costumes "/assets/main/*.png";

    costumes "/assets/main/blank.png", "/assets/main/character.png";

    sounds "/assets/main/*.wav";

    sounds "/assets/main/alert.wav", "/assets/main/character.wav";

    use "file.gs";

// All statement blocks:
say message;
sayfor message, duration;

// All reporter blocks:
round(expr)
