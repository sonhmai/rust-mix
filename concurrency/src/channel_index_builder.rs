/// A concurrent program creating an inverted index.
/// read
/// [whole file as strings]
/// index
/// [in-mem index]
/// merge
/// [large in-mem index]
/// write
/// [index filenames]
/// file merge
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::{fs, io, thread};

struct InMemoryIndex {}

fn start_file_reader_thread(
    documents: Vec<PathBuf>,
) -> (mpsc::Receiver<String>, thread::JoinHandle<io::Result<()>>) {
    let (sender, receiver) = mpsc::channel();
    let documents: Vec<PathBuf> = vec![];

    // read list of files
    // Ownership of sender (not receive) transferred to new thread by move closure.
    let handle = thread::spawn(move || {
        for filename in documents {
            let text = fs::read_to_string(filename)?;
            if sender.send(text).is_err() {
                break;
            }
        }
        Ok(())
    });
    (receiver, handle)
}

/// Spawns a thread that receives String values from 1 channel (texts)
/// and send InMemoryIndex values to another channel (sender/ receiver).
fn start_file_indexing_thread(
    texts: mpsc::Receiver<String>,
) -> (mpsc::Receiver<InMemoryIndex>, thread::JoinHandle<()>) {
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {});
    (receiver, handle)
}

/// Merges indexes in memory
fn start_in_memory_merge_thread(
    file_indexes: mpsc::Receiver<InMemoryIndex>,
) -> (mpsc::Receiver<InMemoryIndex>, thread::JoinHandle<()>) {
    todo!()
}

/// Writes large index to disk
fn start_index_writer_thread(
    big_indexes: mpsc::Receiver<InMemoryIndex>,
    output_dir: &Path,
) -> (mpsc::Receiver<PathBuf>, thread::JoinHandle<io::Result<()>>) {
    todo!()
}

fn merge_index_files(files: mpsc::Receiver<PathBuf>, output_dir: &Path) -> io::Result<()> {
    todo!()
}

/// 40% faster than single-thread equivalent.
/// We clearly haven’t saturated either the system’s I/O capacity or all the CPU cores.
/// What’s going on?
///
/// - Pipelines are like assembly lines in a manufacturing plant:
/// performance is limited by the throughput of the slowest stage.
/// - Measurement shows that the `second stage` is the bottleneck.
/// Our indexing thread uses .to_lowercase() and .is_alphanumeric(),
/// so it spends a lot of time poking around in Unicode tables.
/// - The other stages downstream from indexing spend most of their time asleep in Receiver::recv, waiting for input.
/// - This means we should be able to go faster. As we address the bottlenecks, the degree of parallelism will rise.
fn run_pipeline(documents: Vec<PathBuf>, output_dir: PathBuf) -> io::Result<()> {
    // launch all 5 stages of the pipeline.
    let (texts, h1) = start_file_reader_thread(documents);
    let (pints, h2) = start_file_indexing_thread(texts);
    let (gallons, h3) = start_in_memory_merge_thread(pints);
    let (files, h4) = start_index_writer_thread(gallons, &output_dir);
    let result = merge_index_files(files, &output_dir);

    // wait for threads to finish, holding on to any errors that they encounter.
    // Use .join().unwrap() to explicitly propagate panics from child threads to the main thread.
    let r1 = h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();
    let r4 = h4.join().unwrap();

    // Return first error encountered, if any.
    // (As it happens, h2 and h3 can't fail: those threads
    // are pure in-memory data processing.)
    r1?;
    r4?;
    result
}
