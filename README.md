# Days to double

Or more generally, empirically reporting the number of steps 
between doubling of values.

The program reads a time series which:

- is one value per line
- each of which is a whole number (integer)
- with no missing values
- and made up of equal time steps (e.g., each value is a day, say)

By "doubling", I mean the number of time steps until another value is seen that at least doubles.

I also allow for values decreasing (i.e., halfing). These appear as negavtive values in the output.

If no doubling or halfing is found, a zero is output.
This allows you to compare the input data to the output
with a command such as `paste input output`.

This doubling factor is controlled by a command line flag. 
The default is `-f 2.0`.
