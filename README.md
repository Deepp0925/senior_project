# Untitled Project [proposed: Project Volta]

The main purpose of this project is minimize the time taken to copy (a) file(s)/ folder(s). `Copy` function (which does literal byte copy) in itself is very difficult to optimize as it is I/O bound and vastly depends upon the hardware and the OS. Therefore, the idea is explore listed approaches and/ or combination of them.

> Smart Compression

This will look at the file type(normally the extension) to determine the suitable compression algorithm for best perfmance. This will reduce the number of bytes that need to be copied. Note: that it is possible for compression to take longer time based on given input.

> Multi-Threading

Traditional copying of file is single-thread, which is very inefficient and does not utilize the resources efficiently. Implementing a concurrent(using Async) & parallel I/O operations to minimize the time.

> Determine the appropriate size

There are a lot of overhead associated when opening and closing a file. Generally, large files should be copied in large chunks and small files in small chunks. This drastically reduces the number of disk accesses. The suggestion is gradually growing the size of the buffer from 4KB to 4MB and determining which yields the best performance.

## Resource

This was a part of senior project (as mentioned above), which means all the pre-implementation phase files have been include here as well. This includes the research paper, architecture design, UML class diagram, and UI design.

To understand all the design choices and results take a look at results.
