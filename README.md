# Notes on Advent Of Code 2025

## Day 1 Dial Safe
  Overcame a tricky special case in part2.

## Day 2 Invalid Codes
  Could refactor with a better way of finding interger divisors.

## Day 3 Battery safety limit override
  Straightforward. Part2 the sliding search window was good to figure out.

## Day 4 Forklift trucking
  Part2: Would lend itself a cool animation.

## Day 5 Fresh Products
  Part 2: This years entry which cannot be brute forced.
  
  The solution
  Where the ranges overlap .. one must be expanded, the other must be deleted.
  TODO: 
    Given my solution takes 17 seconds on release mode 
    I should go back and look at the sub-optimal performance.

## Day 6 Multiple Divide

## Day 7 Beam Splitter
  Part one: Nice algorithm using .retain() on a hashSet and building up a list of beams to add 
  as the rows progress.

## Day 8 Extension Coord
  First time using a linked list.
  Part 2 Would not complete until I sort out a major perf optimisation.
    Now using a call to .tuple_combinations() combined with a call to .sort() to 
    effortlessly pass a list of nearest neighbours.
    
  TODO  
  Want to go back and refactor using petgraph
  Visualization -- generate a stl file for all connected points.

## Day 9 Red Square
  Part1: trivial - cartesian product.
  Part2: I need to algorithm that tollerates rays that are parallel to the boundary
  and still counting the winding number correctlty.
  
## Day 10 Button Smasher

  Part1: Fun example, at 3 sec runtime this needs some perf improvements lots 
    of Vec/Stack objects which could be made into arrays with some const generics.
    or I could toggle on a bitvec.
  Part2 test pass but working on input fails. .. This suggests I missed a trick and do not have the fast algorithm.

## Day 11 Loopy

  Lovely little puzzle
  Part 1: Runs fast ... I see I am copying over Vec's of string from one generation to the next 
    maybe I could refactor to manipulate in place. I would be deleting fragments from the middle of the 
    list of fragments. so a linked list?
  Part 2: Has a trick in the tail .. so I learn't the *pathfinding* crate. Learning how to store state in the node 
  was iluminating.
