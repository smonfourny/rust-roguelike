import * as React from "react"

// markup
class IndexPage extends React.Component {
  render() {
    return (
      <main>
        <title>Explore</title>
        <div className="bg-gray-200 h-screen grid place-content-center">
          <p className="px-6 font-bold">Welcome to Explore, traveller!</p>
          <p className="px-6 font-light mb-4">Press the arrow keys to move your character.</p>
          <ul>
            <li className="px-6"><strong>g</strong> - Pick up item</li>
            <li className="px-6"><strong>d</strong> - Drop item</li>
            <li className="px-6"><strong>i</strong> - Open inventory</li>
          </ul>
          <div className="p-6 flex justify-center">
            <div className="flex-shrink-0 shadow-2xl">
              <canvas id="canvas" width="640" height="480" className="rounded-lg"></canvas>
            </div>
          </div>
          <p className="px-6 text-sm font-light">
            Explore is written in Rust and compiled into WebAssembly. This page is best viewed on desktop.
          </p>
          <a href="https://github.com/smonfourny" className="px-6 font-bold text-red-600">
            &gt; Find me on GitHub
          </a>
        </div>
      </main>
    )
  }
}

export default IndexPage

