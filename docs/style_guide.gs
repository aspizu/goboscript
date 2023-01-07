/* Max line length is 88 characters */

/* Multi-line comments should not be indented */
/* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent convallis lobortis
diam, sit amet imperdiet risus cursus a. Donec vel velit vitae orci viverra scelerisque
sed non velit. Pellentesque in quam varius justo pretium sagittis. In hac habitasse
platea dictumst. Sed luctus imperdiet neque ac tincidunt. Sed varius fermentum felis
a rutrum. Praesent egestas nisl mi, eu malesuada nibh interdum vel. Integer tristique,
enim non tincidunt gravida, dolor ante condimentum lacus, eget malesuada lorem orci non
elit. */

/*
  WRONG:
  This is wrong
*/

/* CORRECT:
This is correct */

/* Indent using 2 spaces, tab spaces are not allowed */
/* Opening brackets should be on the same line as the declaration */

/* WRONG: */
onflag
{
    say "Hello";    
}

/* CORRECT: */
onflag {
  say "Hello";
}

/* Use only one costumes, globals and listglobals declaration */
/* Use the style guide for statements here */

costumes "one.svg", "two.svg", "three.svg", "four.svg", "five.svg";

/* Declarations should be separated by a blank line */

onflag {

}

onflag {

}

/* But costumes, globals and listglobals should not be */

costumes "blank.svg";
globals variable;
listglobals list;

/* They should also appear in the order: costumes then globals then listglobals */

/* When functions exceed max line length, indent each argument and place the opening
bracket on a new line */

def FunctionName
  argument1,
  argument2,
  argument3
{
  say "Hello World";
}

/* Commas should follow a white space */

/* CORRECT: */
foo 1, 2, 3;

/* WRONG: */
foo 1,2,3;

/* When statements exceed max line length, indent each argument and place the semi-colon
at the end of the last argument */
FunctionName
  argument1,
  argument2,
  argument3;

/* Infix operators should be separated by a whitespace */

/* WRONG: */

1+1

/* CORRECT: */

1 + 1

/* when expressions exceed max line length */
foo
  $one
  + $two
  + $three;

/* when conditions exceed max line length */
if first_condition
  and second_condition
  and third_condition
{
  ...
} elif this
  and that
{
  ...
}

/* reporters should not have a whitespace before the argument list */
reportername(1, 2, 3)

/* if - elif - else statements */

if condition {
  ...
} elif condition {
  ...
} else {
  ...
}
