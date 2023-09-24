import React from 'react';
import './App.css';
import axios from 'axios';

import CodeMirror from '@uiw/react-codemirror';
import { rust } from '@codemirror/lang-rust';

function App() {
  const [code, setValue] = React.useState(`fn main() {
    println!(\"Hello, world!\");
}`);

  const onChange = React.useCallback((code, viewUpdate) => {
    console.log('code:', code);
    setValue(code);
  }, []);

  const submitCode = () => {
    axios
      .post('http://localhost:9090/submission', {code})
      .then((res) => console.log(res))
  }

  return (
    <div className="grid grid-cols-3 gap-4">
      <div className="my-5 bg-gray-100 rounded-md">
        <p>Create a function that adds two numbers in Rust.</p>
        <p>Return the right answer or you are a disappointment.</p>
      </div>
      <div className="col-span-2 h-screen">
        <div className="bg-gray-100 rounded-md my-5 py-10 px-4 h-4/5">
            <CodeMirror
              className="h-full"
              value={code}
              height="90%"
              width="100%"
              extensions={[rust()]}
              onChange={onChange}
            />
        </div>

        <div className="bg-gray-100 rounded-md">
          <div className="my-5">
            <button className="bg-green-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded my-2 mx-4" onClick={submitCode}>
            Submit
            </button>
          </div>
        </div>
      </div>
    </div>
  )

}

export default App;
