

<a id='definition__r7rs__read-char'></a>

# `read-char` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|byte-or-eof|))`
   * inputs: none;
   * output: a value of type [`byte-or-eof`](../../r7rs/types/byte-or-eof.md#type__r7rs__byte-or-eof);
 * `((|textual-input-port-eof|) |->| (|eof-object|))`
   * input: a value of type [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
   * output: a value of type [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
 * `((|textual-input-port-open|) |->| (|character-or-eof|))`
   * input: a value of type [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);
   * output: a value of type [`character-or-eof`](../../r7rs/types/character-or-eof.md#type__r7rs__character-or-eof);


#### Referenced types

[`byte-or-eof`](../../r7rs/types/byte-or-eof.md#type__r7rs__byte-or-eof);
[`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
[`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
[`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);
[`character-or-eof`](../../r7rs/types/character-or-eof.md#type__r7rs__character-or-eof);


#### Description

> ````
> (read-char)
> (read-char port)
> ````
> 
> 
> Returns the next character available from the textual input `port`,
> updating
> the `port` to point to the following character.  If no more characters
> are available, an end-of-file object is returned.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:ports:input`](../../r7rs/categories/vs_3a_ports_3a_input.md#category__r7rs__vs_3a_ports_3a_input);
[`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);
[`vs:characters`](../../r7rs/categories/vs_3a_characters.md#category__r7rs__vs_3a_characters);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----
