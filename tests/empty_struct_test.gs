// Test empty struct
type empty {}

// This should trigger the empty struct error
list empty mylist;

onflag {
  say length(mylist);
}
