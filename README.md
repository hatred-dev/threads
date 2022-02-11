Implement basic function to split some generic computational work between threads. 
Split should occur only on some threshold - if computational work (input length) is shorter than this threshold, 
no splitting should occur and no threads should be created.

You get as input: 

1. Vec<T>
2. Function f(t: T) -> R


Threshold can be just constant. 

You should return:
   1. Up to you, but probably some Vec of the same length as input(1)

Code should be published on github.