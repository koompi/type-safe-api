import { Todo, User } from "./utils/bindings";
import { rspc } from "./utils/rspc";
import { createSignal } from "solid-js";

function App() {
  let u: User = {
    name: "den1",
    email: "den1@jvpkh.com",
  };
  const [text, setText] = createSignal("");

  let todo = () => {
    return {
      title: text().toString()
    }
  }

  const todos = rspc.createQuery(() => ["todos"]);
  // const createTodo = rspc.createMutation(() => [""])
  const add_todo = rspc.createMutation("create_todos")
  const handle = async () => {
    add_todo.mutate(todo())
    setTimeout(() => todos.refetch(), 1000)

  }
  return (
    <h1>
      <input
        type="text"
        onInput={(e) => {
          setText(e.target.value);
        }}
      />
      <button onClick={handle}>Submit</button>
      <code>

      <pre>{JSON.stringify(todos.data, null, 4)}</pre>
      </code>
    </h1>
  );
}

export default App;
