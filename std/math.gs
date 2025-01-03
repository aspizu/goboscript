# Return the minimum of `A` and `B`.
%define MIN(A,B) ((A) - ((A) - (B)) * ((A) > (B)))

# Return the maximum of `A` and `B`.
%define MAX(A,B) ((A) + ((B) - (A)) * ((A) < (B)))

# Combine RGB values into a single value for use with the `set_pen_color` block.
%define RGB(R,G,B) (((R) * 65536) + ((G) * 256) + (B))

# Combine RGBA values into a single value for use with the `set_pen_color` block.
%define RGBA(R,G,B,A) (((R) * 65536) + ((G) * 256) + (B) + ((A) * 16777216))

# Parse a hexadecimal value.
%define HEX(VALUE) (("0x"&(VALUE))+0)

# Parse a binary value.
%define BIN(VALUE) (("0b"&(VALUE))+0)

# Return `BASE` raised to the power of `EXP`.
%define POW(BASE,EXP) antiln(ln(BASE)*(EXP))

# Return the gamma function of `VALUE`.
%define GAMMA(VALUE) antiln(ln(VALUE)/2.2)

# Clamp `VALUE` above zero. (Returns 0 for `VALUE` < 0)
%define POSITIVE_CLAMP(VALUE) (((VALUE)>0)*(VALUE))

# Clamp `VALUE` below zero. (Returns 0 for `VALUE` > 0)
%define NEGATIVE_CLAMP(VALUE) (((VALUE)<0)*(VALUE))

# Clamp `VALUE` between `MIN` and `MAX`.
# (Returns `MIN` for `VALUE` < `MIN`, `MAX` for `VALUE` > `MAX`)
%define CLAMP(VALUE,MIN,MAX) (((VALUE)>(MIN))*((MAX)+((VALUE)-(MAX))*((VALUE)<(MAX))))

# Return the `N`th bit of `V`'s binary representation.
%define BIT(N, V) (((V) // antiln((N) * ln 2) % 2)

# Return the distance between the points `(X1,Y1)` and `(X2,Y2)`.
%define DIST(X1,Y1,X2,Y2) sqrt(((X2)-(X1))*((X2)-(X1))+((Y2)-(Y1))*((Y2)-(Y1)))

%define ACOSH(X) ln((X)+sqrt((X)*(X)-1))
%define ASINH(X) ln((X)+sqrt((X)*(X)+1))
%define ATANH(X) ln((1+(X))/(1-(X)))/2
%define COSH(X) ((antiln(X)+antiln(-(X)))/2)
%define SINH(X) ((antiln(X)-antiln(-(X)))/2)
%define TANH(X) ((antiln(X)-antiln(-(X)))/(antiln(X)+antiln(-(X))))
%define PI 3.141592653589793
%define E 2.718281828459045
