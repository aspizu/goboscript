# gBlock

[Goboscript Index](../../README.md#goboscript-index) /
`gsc` /
[Sb3](./index.md#sb3) /
gBlock

> Auto-generated documentation for [gsc.sb3.gblock](../../../gsc/sb3/gblock.py) module.

- [gBlock](#gblock)
  - [gArgument](#gargument)
    - [gArgument().serialize](#gargument()serialize)
  - [gBlock](#gblock-1)
    - [gBlock.from_prototype](#gblockfrom_prototype)
    - [gBlock().serialize](#gblock()serialize)
    - [gBlock().serialize_field](#gblock()serialize_field)
    - [gBlock().serialize_fields](#gblock()serialize_fields)
    - [gBlock().serialize_input](#gblock()serialize_input)
    - [gBlock().serialize_inputs](#gblock()serialize_inputs)
  - [gHatBlock](#ghatblock)
    - [gHatBlock().serialize](#ghatblock()serialize)
  - [gList](#glist)
  - [gProcCall](#gproccall)
    - [gProcCall().serialize](#gproccall()serialize)
  - [gProcDef](#gprocdef)
  - [gProcProto](#gprocproto)
    - [gProcProto().serialize](#gprocproto()serialize)
  - [gStack](#gstack)
    - [gStack().serialize](#gstack()serialize)
  - [gVariable](#gvariable)
  - [proccode](#proccode)

## gArgument

[Show source in gblock.py:164](../../../gsc/sb3/gblock.py#L164)

#### Signature

```python
class gArgument(gBlock):
    def __init__(self, name: str, shadow: bool = False):
        ...
```

#### See also

- [gBlock](#gblock)

### gArgument().serialize

[Show source in gblock.py:169](../../../gsc/sb3/gblock.py#L169)

#### Signature

```python
def serialize(self, blocks: gBlockListType, next: str | None, parent: str | None):
    ...
```

#### See also

- [gBlockListType](#gblocklisttype)



## gBlock

[Show source in gblock.py:29](../../../gsc/sb3/gblock.py#L29)

#### Signature

```python
class gBlock:
    def __init__(
        self,
        opcode: str,
        inputs: dict[str, gInputType],
        fields: dict[str, gFieldType],
        comment: str | None = None,
    ):
        ...
```

#### See also

- [gFieldType](#gfieldtype)
- [gInputType](#ginputtype)

### gBlock.from_prototype

[Show source in gblock.py:45](../../../gsc/sb3/gblock.py#L45)

#### Signature

```python
@classmethod
def from_prototype(
    cls, prototype: gPrototype, arguments: list[gInputType], comment: str | None = None
):
    ...
```

#### See also

- [gInputType](#ginputtype)

### gBlock().serialize

[Show source in gblock.py:120](../../../gsc/sb3/gblock.py#L120)

#### Signature

```python
def serialize(self, blocks: gBlockListType, next: str | None, parent: str | None):
    ...
```

#### See also

- [gBlockListType](#gblocklisttype)

### gBlock().serialize_field

[Show source in gblock.py:100](../../../gsc/sb3/gblock.py#L100)

#### Signature

```python
def serialize_field(self, blocks: gBlockListType, value: gFieldType) -> JSON:
    ...
```

#### See also

- [gBlockListType](#gblocklisttype)
- [gFieldType](#gfieldtype)

### gBlock().serialize_fields

[Show source in gblock.py:114](../../../gsc/sb3/gblock.py#L114)

#### Signature

```python
def serialize_fields(self, blocks: gBlockListType):
    ...
```

#### See also

- [gBlockListType](#gblocklisttype)

### gBlock().serialize_input

[Show source in gblock.py:78](../../../gsc/sb3/gblock.py#L78)

#### Signature

```python
def serialize_input(self, blocks: gBlockListType, value: gInputType, name: str) -> JSON:
    ...
```

#### See also

- [gBlockListType](#gblocklisttype)
- [gInputType](#ginputtype)

### gBlock().serialize_inputs

[Show source in gblock.py:108](../../../gsc/sb3/gblock.py#L108)

#### Signature

```python
def serialize_inputs(self, blocks: gBlockListType):
    ...
```

#### See also

- [gBlockListType](#gblocklisttype)



## gHatBlock

[Show source in gblock.py:143](../../../gsc/sb3/gblock.py#L143)

#### Signature

```python
class gHatBlock(gBlock):
    def __init__(
        self,
        opcode: str,
        inputs: dict[str, gInputType],
        fields: dict[str, gFieldType],
        stack: gStack,
    ):
        ...
```

#### See also

- [gBlock](#gblock)
- [gFieldType](#gfieldtype)
- [gInputType](#ginputtype)
- [gStack](#gstack)

### gHatBlock().serialize

[Show source in gblock.py:157](../../../gsc/sb3/gblock.py#L157)

#### Signature

```python
def serialize(self, blocks: gBlockListType, next: str | None, parent: str | None):
    ...
```

#### See also

- [gBlockListType](#gblocklisttype)



## gList

[Show source in gblock.py:23](../../../gsc/sb3/gblock.py#L23)

#### Signature

```python
class gList:
    def __init__(self, name: str, data: list[str] | None = None):
        ...
```



## gProcCall

[Show source in gblock.py:175](../../../gsc/sb3/gblock.py#L175)

#### Signature

```python
class gProcCall(gBlock):
    def __init__(
        self, name: str, inputs: dict[str, gInputType], warp: bool, comment: str | None
    ):
        ...
```

#### See also

- [gBlock](#gblock)
- [gInputType](#ginputtype)

### gProcCall().serialize

[Show source in gblock.py:183](../../../gsc/sb3/gblock.py#L183)

#### Signature

```python
def serialize(self, blocks: gBlockListType, next: str | None, parent: str | None):
    ...
```

#### See also

- [gBlockListType](#gblocklisttype)



## gProcDef

[Show source in gblock.py:220](../../../gsc/sb3/gblock.py#L220)

#### Signature

```python
class gProcDef(gHatBlock):
    def __init__(self, name: str, arguments: list[Token], warp: bool, stack: gStack):
        ...
```

#### See also

- [gHatBlock](#ghatblock)
- [gStack](#gstack)



## gProcProto

[Show source in gblock.py:196](../../../gsc/sb3/gblock.py#L196)

#### Signature

```python
class gProcProto(gBlock):
    def __init__(self, name: str, arguments: list[Token], warp: bool):
        ...
```

#### See also

- [gBlock](#gblock)

### gProcProto().serialize

[Show source in gblock.py:206](../../../gsc/sb3/gblock.py#L206)

#### Signature

```python
def serialize(self, blocks: gBlockListType, next: str | None, parent: str | None):
    ...
```

#### See also

- [gBlockListType](#gblocklisttype)



## gStack

[Show source in gblock.py:137](../../../gsc/sb3/gblock.py#L137)

#### Signature

```python
class gStack(list[gBlock]):
    ...
```

#### See also

- [gBlock](#gblock)

### gStack().serialize

[Show source in gblock.py:138](../../../gsc/sb3/gblock.py#L138)

#### Signature

```python
def serialize(self, blocks: gBlockListType, parent: str):
    ...
```

#### See also

- [gBlockListType](#gblocklisttype)



## gVariable

[Show source in gblock.py:19](../../../gsc/sb3/gblock.py#L19)

#### Signature

```python
class gVariable(str):
    ...
```



## proccode

[Show source in gblock.py:14](../../../gsc/sb3/gblock.py#L14)

#### Signature

```python
def proccode(name: str, inputs: dict[str, "gArgument"]):
    ...
```