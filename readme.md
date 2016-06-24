Rusty Lisp
==============
Simple implementation of `lisp` written in `rust`.

Features:
--------
 - function expression `fn*`
 - create new environments `let*`
 - if expressions
 - printing

Example:
-------
```
> (def! a 6)
=> ()
> (+ a 5)
=> 11
> (def! id (fn* [a] a))
=> ()
> (def! square (fn* [a] (* a a)))
=> ()
> (if (= (square 5) 25) (print "Square of 5 is 25"))
"Square of 5 is 25"
=> ()
> (let* (a 2) a)
=> 2
> (if (= 1 2) "TRUE" "FALSE")
=> "FALSE"

```