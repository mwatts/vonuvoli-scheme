

<a id='type__r7rs__binary-port-open'></a>

# `binary-port-open` -- `r7rs` Types


#### Sub-types tree

* **[`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open)**:
  * **[`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof)**;
* **[`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open)**;


#### Super-type

[`binary-port`](../../r7rs/types/binary-port.md#type__r7rs__binary-port);
[`port-open`](../../r7rs/types/port-open.md#type__r7rs__port-open);


##### Super-types recursive

[`port`](../../r7rs/types/port.md#type__r7rs__port);


#### Sub-types

[`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open);
[`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open);


##### Sub-types recursive

[`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);


#### Predicate

```
(|lambda| (|value|) (|and| (|binary-port?| |value|) (|or| (|input-port-open?| |value|) (|output-port-open?| |value|))))
```


#### Categories

[`r7rs:types-ports`](../../r7rs/categories/r7rs_3a_types-ports.md#category__r7rs__r7rs_3a_types-ports);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----
