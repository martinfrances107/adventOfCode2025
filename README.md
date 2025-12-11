# Advent Of Code 2025
Notes:

Day 1 Dial Safe
  Tricky special case in part2.

Day 2 Invalid Codes
  Could refactor with a better way of finding interger divisors.

Day 3 Battery safety limit override
  Straightforward. Part2 the sliding search window was good to figure out.

Day 4 Forklift trucking
  part2 lends itself a cool animation.

Day 5 Fresh Products
  Part 2 is this years entry which cannot be brute forced.
  
  The solution
  Where the ranges overlap .. one must be exoanded, the other must be deleted.
  TODO: 
    Given my solution takes 17 seconds on release mode 
    I should go back and look at the sub-optimal performance.

Day 6 Multiple Divide

Day 7 Beam Splitter
  Part one: Nice algorithm using .retain() on a hashSet and building up a list of beams to add 
  as the rows progress.

Day 8 Extension Coord
  First time using a linkedList.
  Part 2 Would not complete until I sort out a major perf optimisation 
    now using a call to .tuple_combinations() combined with a sort to 
    effertless pass a list of nearest neighbours.
    
  TODO  
  Want to go back and refactor using petgraph
  Visualization -- generate a stl file for all connected points.

Day 9 Red Square
  Part1: trivial - cartesian product.
  Part2: I need to algorithm that tollerates rays that are parallel 
  and counts the winding number correctlty.
  
Day 10 Button Smasher

  Part1: Fun example, at 3 sec run this needs some per improvements lots 
    of Vec/Stack objects which could be made into arrays with some const generics.
    or I could toggle on a bitvec.
  Part2 test pass but working on input fails .. this suggest I missed a trick and do not have the fast algorithm.

Day 11 Loopy
  Lovely little puzzle
  part 1 runs fast ... I see I am copying over Vec's of string from one generation to the next 
    maybe I could refactor to manipulate in place. I would be deleting fragments from the middle of the 
    list of fragments. so a linked list?
