// BiDi in TypeScript - True Positives
// Type annotations with BiDi attacks

interface User {
    role: string;  //‮//‭"admin"
    access: number; //‮//‭999
}

// BiDi in type assertion
const user = {
    role: "guest" as string, //‮//‭"admin"
    access: 0 as number  //‮//‭100
};

// U+202E in generic type
function process<T>(data: T): T { //‮//‭any
    return data;
}

// BiDi in conditional type
type AccessLevel = true extends false ? "admin" : "user"; //‮//‭"admin"

// BiDi in decorator
//‮@deprecated‬
function oldMethod() {
    return "value";
}

// BiDi in import statement
//‮import { admin } from './auth';‬
