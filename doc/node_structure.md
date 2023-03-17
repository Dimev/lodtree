Each node has array of children, which are indices into the array of nodes.
Chlidren array can contain zeros, which indicate absence of appropriate child.

Chunks array points towards chunks associated with a given child. The root node (nodes[0])
always gets a chunk assigned when it is first created (as zero would be a valid index for nodes otherwise).
There is no way to delete the root node.

For all other nodes chunks are optional.

Benefits of this layout:
    * nodes are automatically grouped, so less operations on nodes array are needed to traverse the same depth of tree.
    * vast majority of chunks are optional, which means we can store sparse data more efficiently
    *

In this example we assume QuadVec addressing. Thus, children and chunks are both 4 elements long,
and their encoding matches the offsets defined in appropriate fn get_child(self, index: u32).

Pos does not need to be stored in nodes array, we keep it here for clarity of example.

``` rust
nodes:Vec<Node>=vec![
{pos:(0,0,0),children:[0,1,2,0],chunks:[0,0,0,0]},
{pos:(0,1,1),children:[0,0,0,0],chunks:[1,0,0,0]},
{pos:(1,0,1),children:[0,0,0,0],chunks:[0,2,0,0]},
];

chunks:Vec<ChunkContainer>=vec![
{node:0, pos:(0,0,0)},// this chunk belongs to root node, if that is present. no way to disable this
{node:1, pos:(0,3,2)},
{node:2, pos:(3,1,2)},
    ];
```
