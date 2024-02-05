# Mental Map for memory management
This is just a mental map for myself to manage memory.
## Chunk
A chunk is represented as a byte array of 512 bytes. This leads to a chunk being 64x64 'pixels'.
### Example
$$

\text{Chunk represented as a matrix for ease of mind:} \\
\begin{align}
\text{C} = \begin{bmatrix}
c_{0} & c_{1} & \cdots & c_{7} \\
c_{8} & c_{9} & \cdots & c_{15} \\
\vdots & \vdots & \ddots \\
c_{504} & c_{505} & & c_{511}
\end{bmatrix} \\
\end{align} \\
\text{Consider following:} \\
\begin{align}
x = 12 \\
y = 1 \\
\end{align} \\
\text{We can then access the corresponding byte:} \\
\begin{align}
i_{byte} &= 8y + x/8 \\
i_{byte} &= 8 + 1 = 9 \\
\end{align} \\
\text{We then obtain the bit index by performing a modulo operation:} \\
\begin{align}
i_{bit} &\equiv x \pmod{8} \\
i_{bit} &\equiv 12 \pmod{8} = 4 \\
\end{align} \\
\text{Consider this:} \\
\begin{align}
c_{9} = 10101010_{2} \\
\end{align} \\
\text{And then access the corresponding bit by bitwise operations:} \\
\begin{align}
b &= (c_{i} \mathbin{\gg} i_{bit}) \mathbin{\&} 1 \\
b &= (c_{9} \mathbin{\gg} 4) \mathbin{\&} 1 \rightarrow 1010\textcolor{red}{1}010 \\
b &= 1
\end{align}

$$