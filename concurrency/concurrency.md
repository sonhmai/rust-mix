
3 techniques for using threads
1. fork join
2. channel
3. shared mutable state with locks

## Fork-Join Parallelism

![img_3.png](img_3.png)

## Channels

![19_5_channel for strings](channel_strings.png)

![19_6_unix_pipeline.png](img_2.png)

Rust channels are faster than Unix pipes. 
- Sending a value `moves it rather than copying it`, 
- moves are fast even when you’re moving data structures that contain many megabytes of data.

![19_7_index_builder_pipeline.png](img_1.png)

![19_9_send_sync](send_and_sync_types.png)

Thread safety
- `Send`: safe to pass by value to another thread
- `Sync`: safe to pass by non-mut ref to another thread

![19_10_rc_string](img.png)

Why `Rc<String>` not `Sync` nor `Send`?
- What would happen if Rc<String> were Sync?
  - allowing threads to share a single Rc via shared references.
  - If both threads happen to try to clone the Rc at the same time, as shown in Figure 19-10, we have a data race as both threads increment the shared reference count.

## Shared Mutable State

```
1. Mutex
2. RwLock<T>
3. Condvar
4. Atomic
```

![19_11_read_complex_type.png](img_4.png)

