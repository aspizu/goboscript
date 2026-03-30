### Generating Random Decimal Numbers

`random` only outputs decimals if its arguments are formatted as decimals, but goboscript optimizes `1.0` to `1`, turning them into integers. Pass the arguments as strings to prevent this:

```goboscript
say random("0.0", "1.0");
```

### Converting Strings to Numbers

Scratch arithmetic operators cast strings to numbers automatically. To force this in goboscript, use a string zero or empty string — the optimizer ignores these, unlike `+ 0`:

```goboscript
numeric_value = string_variable + "0";
numeric_value = string_variable + "";
```

### Simulating "Wait Until"

Use an `until` loop with an empty body:

```goboscript
until (condition) {}
```

### Encapsulating Variables

Use `%define` to alias a long unique name to a short local one, then `%undef` it at the end of the file so other files can't access it directly.

`lib/my_module.gs`

```goboscript
%define private_variable __my_module__private_variable

func get_my_variable() {
    return private_variable;
}

%undef private_variable
```

`main.gs`

```goboscript
%include lib/my_module

onflag {
    say get_my_variable();
    # private_variable = 10; # error — alias no longer defined
}
```

### "While" Loop Macro

Goboscript has no `while` loop, but you can macro it:

```goboscript
%define while(CONDITION) until (not (CONDITION))

onflag {
    i = 0;
    while (i < 10) {
        i++;
    }
}
```
