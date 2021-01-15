const React = require("react");

exports.onRenderBody = ({setPostBodyComponents}) => {
    setPostBodyComponents([
      <script key="1" src={'wasm/explore.js'} type="text/javascript" />,
      <script key="2" src={'wasm/binding.js'} type="text/javascript" />,
    ]);
};