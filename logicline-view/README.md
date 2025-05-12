# logicline-view

A library used to display [logic line](https://github.com/roboplc/logicline)
state snapshots.

```react-tsx
import { useState, useEffect } from "react";
import { RackView, type Snapshot } from "logicline-view";
import "../node_modules/logicline-view/dist/style.css";

const App = () => {
  const [data, setData] = useState<Snapshot>({ lines: [] as any });

  useEffect(() => {
    let t: number;
    const fetchData = () => {
      fetch(`http://${window.location.hostname}:9001/state`)
        .then((res) => res.json())
        .then(setData);
      t = setTimeout(fetchData, 500);
    };
    fetchData();

    return () => clearTimeout(t);
  }, []);
  return (
    <RackView
      data={data}
      onBlockClick={(v) => {
        console.log(v);
      }}
    />
  );
};
```

## Quick example

## Style customization

The following variables can be used to customize basic colors.

```css
--logicline-secondary-text-color: #0a1a2c;
--logicline-decorator-color: #ccc;
--logicline-accent-color: #009e23;
--logicline-disabled-color: #555;
```

To perform more customization, override the provided CSS classes.
