/* Copyright 2021 mrcl dbbrk
 * SPDX-License-Identifier: Apache-2.0
 */


/* SparseTree */
/******************************************************************************/

pub struct SparseTree<V: PartialEq + Copy, const N: usize> (Tree<V, N>, usize);

impl<V: PartialEq + Copy, const N: usize> SparseTree<V, N> {
    pub fn new(depth: u32, val: V) -> SparseTree<V, N> {
        assert!(depth > 0);
        SparseTree(
            Tree::Leaf(val),
            if depth == 1 {0} else {2usize.pow(depth - 2)}
        )
    }

    pub fn put(&mut self, key: [usize; N], val: V) {
        for i in key { assert!(self.1 > i / 2); }
        self.0.put(self.1, key, val);
    }

    pub fn get(&self, key: [usize; N]) -> V {
        for i in key { assert!(self.1 > i / 2); }
        self.0.get(self.1, key)
    }

    pub fn swap(&mut self, key: [usize; N], val: V) -> V {
        for i in key { assert!(self.1 > i / 2); }
        self.0.swap(self.1, key, val)
    }
}

#[derive(PartialEq)]
enum Tree<V: PartialEq + Copy, const N: usize>
    { Branch(Box<[Tree<V, N>]>), Leaf(V) }

impl<V: PartialEq + Copy, const N: usize> Tree<V, N> {

    fn sector(key: &mut [usize; N], bisector: &mut usize) -> usize {
        let mut sct = 0;
        for i in 0..N {
            if key[i] >= *bisector {
                key[i] -= *bisector;
                sct += 1<<i;
            }
        }
        *bisector /= 2;
        sct
    }

    fn put(&mut self, mut bisector: usize, mut key: [usize; N], val: V) {
        match self {
            Tree::Leaf(leaf) => {
                if *leaf != val {
                    if bisector == 0 {
                        *leaf = val;
                    } else {
                        let mut leafs = Vec::new();
                        for _ in 0..(2<<N) {leafs.push(Tree::Leaf(*leaf));}
                        *self = Tree::Branch(leafs.into_boxed_slice());
                        self.put(bisector, key, val);
                    }
                }
            }
            Tree::Branch(branch) => {
                let sct = Self::sector(&mut key, &mut bisector);
                branch[sct].put(bisector, key, val);
                if let Tree::Leaf(v) = branch[0] {
                    for t in branch[1..2<<N].iter() {
                        if *t != branch[0] { return; }
                    }
                    *self = Tree::Leaf(v);
                }
            }
        }
    }

    fn get(&self, mut bisector: usize, mut key: [usize; N]) -> V {
        match self {
            Tree::Leaf(leaf) => { *leaf }
            Tree::Branch(branch) => {
                let sct = Self::sector(&mut key, &mut bisector);
                branch[sct].get(bisector, key)
            }
        }
    }

    fn swap(&mut self, mut bisector: usize, mut key: [usize; N], mut val: V) -> V {
        match self {
            Tree::Leaf(leaf) => {
                let ret = *leaf;
                self.put(bisector, key, val);
                ret
            }
            Tree::Branch(branch) => {
                let sct = Self::sector(&mut key, &mut bisector);
                val = branch[sct].swap(bisector, key, val);
                if let Tree::Leaf(v) = branch[0] {
                    for t in branch[1..2<<N].iter() {
                        if *t != branch[0] { return val }
                    }
                    *self = Tree::Leaf(v);
                }
                val
            }
        }
    }
}
