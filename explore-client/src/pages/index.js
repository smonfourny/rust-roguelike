import * as React from "react"

// markup
class IndexPage extends React.Component {
  render() {
    return (
      <main>
        <title>Explore</title>
        <div className="bg-gray-200 h-screen grid place-content-center">
          <p className="px-6 font-bold">Welcome to Explore, traveller!</p>
          <p className="px-6 font-light">Press the arrow keys to move your character.</p>
          <div className="p-6 flex justify-center">
            <div className="flex-shrink-0 shadow-2xl">
              <canvas id="canvas" width="640" height="480" className="rounded-lg"></canvas>
            </div>
          </div>
        </div>
      </main>
    )
  }
}

export default IndexPage

