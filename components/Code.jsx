import React, { useState, useEffect } from "react";
import Prism from "prismjs";
import "prismjs/themes/prism-tomorrow.min.css";
import "prismjs/components/prism-rust.min.js";
import "prismjs/components/prism-jsx.min.js";
import "prismjs/components/prism-python.min.js";

export default function Code({ code, language }) {
  const [highlighted, setHighlighted] = useState(null);

  useEffect(() => {
    if (highlighted == null) {
      const data = Prism.highlight(code, Prism.languages[language], language);
      const inner = <div dangerouslySetInnerHTML={{__html: data}} />;
      setHighlighted(inner);
    }
  });

  return (
    <div className="Code">
      <pre>
        <code className={`language-${language}`}>
          {highlighted}
        </code>
      </pre>
    </div>
  );
}

