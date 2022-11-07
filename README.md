## References
[IANA DNS Parameters](https://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml)

[DNS Packet format basics](https://mislove.org/teaching/cs4700/spring11/handouts/project1-primer.pdf)

## Getting started

Program overview:

1. Create a DNS packet
2. Send it to a DNS server
3. Receive a response
4. Parse the response

I've already done the tedious part of this - defining all the structure members for the DNS packet.

It would be worth taking a look at the references above though, as I haven't tagged some of the
enum members with their values.

The remaining work to be done to get the program to run is:

1. Implement a function that takes a string and a query type and returns a DNS packet
2. Implement a function to serialise a DNS packet into a u8 vector
3. Implement a function to deserialise a u8 vector into a DNS packet

This will allow the program to send the query and receive the response, because the DNS request and 
response format are the same, except a response will have answers on the end.

There are minimal test cases for each of the functions above. Feel free to add more.

Twiddling the DNS flag bits is tedious - or at least, I haven't found a nice expressive way to do
it in rust yet. You can skip that if you like by using constant u16 values for common configurations
of DNS flags (Generally you are either sending a query or a response, and not dealing with truncation, 
you always want recursion, you're never going to use the reserved bit, etc.)

## Extensions

1. Use the program to get a valid response, and use that as a test case for response parsing.

This will be tricky as you need to implement DNS "compression" mechanism (see references above)

2. Perform recursive queries (i.e. implement a full resolver)

This means sending queries with recursion desired set to false, and using the responses from
queries to perform further queries to get a DNS answer.