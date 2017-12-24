Declaration of common CG vector struct with named elements,
devoid of any other stylistic choices;
Exists to decouple data declarations from details of maths libraries.
If this crate already exists, I will happily delete it and reference, but I dont want to commit to a whole *vector lib*, just the shared struct.

e.g. in my own setup,

               Vec3<T>
           /               \
          |                   Array3d - uses vec<int> indices
          |                     but doesn't need float-VecMath
    float VecMath              Could share Array3d without VecMath
           |
     3d rendering etc



All the methods of representing vector maths have different advantages/disadvantages

The merit of ```struct Vec3{x,y,z}`` is: it's visually distinct from array access; graphics code involves indexing 'arrays of vectors'. It's possible to keep coordinate fields and collections of coordinates visually distinct, which is helpful in code with various levels of indexing going on.

Tuple structs are appealing (for their default constructor) but .0 .1 need more spacing when nested (e.g. matrices)

inbuilt [T;N] arrays are great but visually confusable with variable-size collection access.


Use of seperately parameterized X,Y,Z,W component types:-

Might seem like overkill but..
These default to the more common 'single shared type', but the versatility is there to plug in semantic 'zero/one' types which would take no storage, allowing you to represent homogeneous points/vectors with W=One/Zero whilst only taking 3 fields in memory. 

Alternatively one might demand seperate precision, eg. 2d map coordinates versus heights, screen coordinates versus depth buffer values