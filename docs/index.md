# goboscript

![image](https://shields.io/crates/l/goboscript)

goboscript is a text-based programming language which compiles to Scratch. It allows
you to write Scratch projects in text, and compile it into a .sb3 file - which can be
opened in the Scratch editor, TurboWarp or be uploaded to the Scratch website.

goboscript allows you to create advanced Scratch projects with ease, you can use any
text editor, use a version control system such as git. You can refactor your code
using search and replace. Text code can be copy pasted, which allows you to easily reuse
code or share it with others. goboscript syntax is concise and easy to read.

goboscript allows you to integrate external tooling and workflows, such as using a
script to generate costumes for a text rendering engine. Or loading in images into
lists.

goboscript has a powerful macro system - similar to Rust's macro system. This allows
you to write macros to generate code.

goboscript is more than just an 1:1 mapping of Scratch blocks to text, it also has
additional features like local variables for procedures (custom blocks).

goboscript also performs optimizations, detects problems and unused code.

|goboscript|Scratch|
|---|---|
|![](https://media.discordapp.net/attachments/1129589844637667431/1221342945160986674/image.png?ex=66123b41&is=65ffc641&hm=7a9e2dc6d578bb23215bb870a5a9a2297b6b93aa7259dcb3c57030c112c73f23&=&format=webp&quality=lossless)|![](https://media.discordapp.net/attachments/1129589844637667431/1221343388117372980/image.png?ex=66123bab&is=65ffc6ab&hm=94597db42e084f4740f4134489cb8aa437611240f75d6125263d20163e94181b&=&format=webp&quality=lossless)|
