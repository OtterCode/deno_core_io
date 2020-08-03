console.log("Fired this");
var ops = Deno.core.ops();
var first = Deno.core.dispatch(ops["get_string"]);

var first_string = utf8ArrayToString(first);

console.log(first_string);

console.log("Transforming string");

var second_string = first_string.toUpperCase();

var res = Deno.core.dispatch(ops["return_string"], to_uint8array(second_string));
