console.log("Fired this");
var ops = Deno.core.ops();
var first = Deno.core.dispatch(ops["get_string"]);

var first_string = utf8ArrayToString(first);

console.log(first_string);

console.log("Returning string");

var res = Deno.core.dispatch(ops["return_string"], first);
