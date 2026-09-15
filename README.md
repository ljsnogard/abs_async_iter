# abs_async_iter

Provide `TrAsyncIterator` and `impl TrAsyncIterator for core::iter::Iterator`.  

This is different from `std::AsyncIter` in that, when calling `next_async`, 
caller can explicitly cancel the `await`ing.

```rust
while let Option::Some(item) = iter.next_async().may_cancel_with(cancel).await {
    todo!()
}
```
