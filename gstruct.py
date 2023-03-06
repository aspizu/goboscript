struct_name = "foo"
struct_members = ["alpha", "beta", "gamma", "delta"]


print(f"def {struct_name}_init {{")
for index, member_name in enumerate(struct_members):
    print(f"    {struct_name}_{member_name} = {1+index};")
print(f"    {struct_name}_length = {1+len(struct_members)};")
print(f"    {struct_name}_deleted[];")
print(f"    {struct_name}[];")
print("}\n")

print(f"def {struct_name}_new {', '.join(struct_members)} {{")
print(f"    if {struct_name}_deleted.length = 0 {{")
print(f"        {struct_name}.add 0;")
print("}\n")
