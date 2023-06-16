import { User } from "./utils/bindings";
import { rspc } from "./utils/rspc";

function App() {
  let u: User = {
    name: "den1",
    email: "den1@jvpkh.com",
  };
  const getName = rspc.createQuery(() => ["name"]);
  const users = rspc.createQuery(() => ["users", u]);
  return <h1>Hello world!!!! You are {JSON.stringify(users.data)}</h1>;
}

export default App;
