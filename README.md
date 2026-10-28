<h1>Dice Macros</h1>
This compiler can compile and run dice syntax for RPGs and Wargames, e.g., 1d6 would roll 1 six-sided die. 
It can be interacted with via the command line. 
<h2>Command Line Arguments</h2>
Invoke with <code>cargo run --release</code> in the directory containing the remote repository.
Then <code> -- </code>, followed by the arguments:
<ul>
  <li><code>-h</code> help</li>
  <li><code>-f <FILE></code> - compile from a text file</li>
  <li><code>-c <INPUTSTRING></code> - write a line of code to the command line to compile. You can use -f OR -c</li>
  <li><code>-o <FILENAME></code> - (optional) provide or create a file to store the compiled code in</li>
  <li><code>-i</code> - (optional) run immediately after compiling. If no -o is given, this is always true. If -o is given, it defaults to false</li>
</ul>

<h2>Syntax</h2>
XdN - rolls X N-sided dice and sums them, where X and N are positive integers<br>
XdNkY - as above, but sums only the highest Y dice<br>
XdNdlY - as above, but drops the lowest Y dice and does not sum them<br>
+,-,/,* - arithmetic operators. Applying an operator to a dice roll takes the sum of the rolls as an operand.<br>
Please note the compiler does not support negative or fractional values at this time; as such, / operator may cause a crash. This is being worked on.<br>
All numbers used must be within the range 0-255 inclusive.
<br><br>
If you find an error in the compiler, please fill out an Issue with the full compiler output and your input
