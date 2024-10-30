# Rust Data Structures Comprehensive Guide

## Detailed Explanations

### Vec<T> (Vector)
A vector is a resizable array implemented as a contiguous block of memory. Think of it as a dynamic array that can grow or shrink as needed.

#### How it works internally:
```rust
struct Vec<T> {
    ptr: *mut T,        // Pointer to the data
    len: usize,         // Current length
    capacity: usize,    // Total allocated capacity
}
```

- When elements are added, if `len == capacity`, Vec allocates a new larger block (typically 2x size)
- Elements are stored sequentially in memory
- Removal from the end is O(1) as it just decreases length
- Removal from middle is O(n) as it needs to shift elements

#### Visual Representation:
```
Memory: [1][2][3][4][_][_][_][_]
         ^
         ptr
len: 4
capacity: 8
```

### VecDeque<T> (Double-ended queue)
A double-ended queue implemented as a ring buffer, allowing efficient insertion and removal at both ends.

#### How it works internally:
```rust
struct VecDeque<T> {
    buf: *mut T,        // Circular buffer
    head: usize,        // Start index
    tail: usize,        // End index
    capacity: usize,    // Total size
}
```

#### Visual Representation:
```
Ring Buffer:  [3][4][_][_][1][2]
                       ^   ^
                     tail head
```
- When the end is reached, it wraps around to the beginning
- Grows by reallocating and reorganizing when full
- Maintains indices for head and tail of queue

### HashMap<K, V>
A hash table implementation that stores key-value pairs using a hashing function to determine element locations.

#### How it works internally:
```rust
struct HashMap<K, V> {
    table: Vec<Option<(K, V)>>,
    len: usize,
    capacity: usize,
}
```

#### Visual Representation:
```
Hash Table:
[0] -> ("apple", 5)
[1] -> None
[2] -> ("banana", 3) -> ("orange", 2)  // (collision handling)
[3] -> None
[4] -> ("pear", 1)
```

- Uses SipHash-1-3 as default hasher for DoS protection
- Handles collisions using linear probing
- Automatically resizes when load factor is too high

### HashSet<T>
A set implementation based on HashMap where only keys are stored, ensuring uniqueness.

#### How it works internally:
```rust
struct HashSet<T> {
    map: HashMap<T, ()>,  // Values are empty tuples
}
```

#### Visual Representation:
```
Hash Table:
[0] -> "apple"
[1] -> None
[2] -> "banana" -> "orange"  // (collision handling)
[3] -> None
[4] -> "pear"
```

### BTreeMap<K, V>
A sorted map implemented as a B-tree, maintaining keys in sorted order.

#### How it works internally:
- Each node contains multiple key-value pairs
- Tree is balanced automatically
- Nodes split when too full, merge when too empty

#### Visual Representation:
```
       [10, 20]
      /    |    \
[5,7,8] [15,16] [25,30]
```

### BinaryHeap<T>
A priority queue implemented as a max-heap, where the largest element is always at the top.

#### How it works internally:
```rust
struct BinaryHeap<T> {
    data: Vec<T>,  // Heap stored as array
}
```

#### Visual Representation:
```
        100
       /    \
     50      40
    /  \    /
   30   20 10
```

- Implemented as a binary max-heap
- Parent is always larger than children
- Uses array representation where:
  - Left child = 2*i + 1
  - Right child = 2*i + 2
  - Parent = (i-1)/2

### String
A UTF-8 encoded, growable string type.

#### How it works internally:
```rust
struct String {
    vec: Vec<u8>,  // UTF-8 encoded bytes
}
```

#### Visual Representation:
```
String "Hello":
[72,101,108,108,111]
```

- Guarantees valid UTF-8 encoding
- Stores bytes, not characters
- Length is number of bytes, not characters

## Performance Characteristics

| Data Structure | Operation      | Average Case | Worst Case |
|---------------|----------------|--------------|------------|
| Vec           | Push/Pop (end) | O(1)         | O(n)*      |
|               | Insert/Remove  | O(n)         | O(n)       |
|               | Index access   | O(1)         | O(1)       |
| VecDeque      | Push/Pop (ends)| O(1)         | O(n)*      |
|               | Insert/Remove  | O(n)         | O(n)       |
|               | Index access   | O(1)         | O(1)       |
| HashMap       | Insert         | O(1)         | O(n)       |
|               | Remove         | O(1)         | O(n)       |
|               | Search         | O(1)         | O(n)       |
| BTreeMap      | Insert         | O(log n)     | O(log n)   |
|               | Remove         | O(log n)     | O(log n)   |
|               | Search         | O(log n)     | O(log n)   |
| BinaryHeap    | Insert         | O(log n)     | O(log n)   |
|               | Pop max        | O(log n)     | O(log n)   |
|               | Peek max       | O(1)         | O(1)       |

*When reallocation is needed

## Memory Layout

### Contiguous Memory Structures
1. Vec<T>
   ```
   Stack:     Heap:
   [ptr] ---> [element][element][element]...
   [len]
   [capacity]
   ```

2. String (Vec<u8> internally)
   ```
   Stack:     Heap:
   [ptr] ---> [byte][byte][byte]...
   [len]
   [capacity]
   ```

### Non-contiguous Structures
1. HashMap<K,V>
   ```
   Stack:          Heap:
   [ptr] ------> [bucket][bucket][bucket]...
   [len]            |
   [capacity]       v
                  [entry]->[entry]->...
   ```

2. BTreeMap<K,V>
   ```
   Stack:          Heap:
   [ptr] ------> [node]
   [len]           |
                   v
                [node]->[node]->...
   ```

## Common Use Case Examples

### Vec<T>
```rust
// Dynamic array of scores
let mut scores = Vec::new();
scores.push(10);  // O(1) append
scores[0];        // O(1) access
```

### VecDeque<T>
```rust
// Processing queue
let mut queue = VecDeque::new();
queue.push_back(1);    // Add to end
queue.pop_front();     // Remove from front
```

### HashMap<K,V>
```rust
// Cache implementation
let mut cache = HashMap::new();
cache.insert("key", value);    // O(1) insert
cache.get("key");             // O(1) lookup
```

### BinaryHeap<T>
```rust
// Priority tasks
let mut tasks = BinaryHeap::new();
tasks.push(Task { priority: 5 });
tasks.pop();  // Get highest priority
```

## Selection Guidelines (Extended)

### Choose Vec<T> when:
- You need cache-efficient sequential access
- Most operations are push/pop at the end
- Random access by index is frequent
- Memory locality is critical

Example scenarios:
- Game entity lists
- Stack implementations
- Building dynamic lists
- Buffering data

### Choose VecDeque<T> when:
- You need efficient operations at both ends
- Implementing a queue or deque
- Working with sliding windows
- Need ring buffer behavior

Example scenarios:
- Message queues
- Task scheduling
- History tracking
- Stream processing

### Choose HashMap<K,V> when:
- Fast lookups are critical
- Data has natural key-value relationships
- Order doesn't matter
- You need constant-time access

Example scenarios:
- Caches
- Counting frequencies
- Symbol tables
- Fast lookups

### Choose BTreeMap<K,V> when:
- You need ordered iteration
- Range queries are common
- Memory efficiency is important
- You need predecessor/successor queries

Example scenarios:
- Database indices
- Ordered mappings
- Range queries
- Memory-constrained systems

## Best Practices

1. Initialization
```rust
// Preallocate when size is known
let mut vec = Vec::with_capacity(1000);

// Use type inference when possible
let mut map = HashMap::new();
```

2. Iteration
```rust
// Prefer iter() for reading
for item in vec.iter() {}

// Use iter_mut() for modification
for item in vec.iter_mut() {}
```

3. Memory Management
```rust
// Shrink to fit after large removals
vec.shrink_to_fit();

// Clear without deallocating
vec.clear();
```

4. Accessing Elements
```rust
// Safe access with get()
if let Some(value) = map.get(&key) {}

// Mutation with entry API
map.entry(key).or_insert(value);
```

## Common Pitfalls to Avoid

1. Vec<T>
- Frequent insertions at beginning
- Not preallocating when size is known
- Unnecessary cloning

2. HashMap<K,V>
- Using non-Hash types as keys
- Not handling potential absence of keys
- Forgetting about DoS protection overhead

3. String
- Indexing directly into UTF-8
- Assuming character count equals byte length
- Inefficient string concatenation

4. BinaryHeap<T>
- Using for FIFO queues
- Not implementing Ord correctly
- Assuming sorted iteration
