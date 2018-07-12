

<a id='definition__r7rs__delay-force'></a>

# `delay-force` -- `r7rs` Definitions


#### Kind

`syntax`;


#### Syntax signature

Syntax keywords:
 * `expression`: expression;

Syntax variants:
 * `(|_| |expression|)`


#### Description

> ````
> (delay-force <expression>)
> ````
> 
> 
> **Semantics**:
> The expression `(delay-force expression)` is conceptually similar to
> `(delay (force expression))`,
> with the difference that forcing the result
> of `delay-force` will in effect result in a tail call to
> `(force expression)`,
> while forcing the result of
> `(delay (force expression))`
> might not.  Thus
> iterative lazy algorithms that might result in a long series of chains of
> `delay` and `force`
> can be rewritten using `delay-force` to prevent consuming
> unbounded space during evaluation.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:lazy`](../../r7rs/categories/r7rs_3a_lazy.md#category__r7rs__r7rs_3a_lazy);
[`vs:promises`](../../r7rs/categories/vs_3a_promises.md#category__r7rs__vs_3a_promises);
[`vs:evaluator`](../../r7rs/categories/vs_3a_evaluator.md#category__r7rs__vs_3a_evaluator);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----
