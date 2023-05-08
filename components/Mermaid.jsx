import mermaidAPI from 'mermaid'
import Script from 'next/script'
import { useState, useEffect} from 'react'
import Children from 'react-children-utilities'

function Mermaid({ title, children }) {

  const [diagram, setDiagram] = useState(null);
  useEffect(() => {
    async function renderDiagram() {
      var graph = '';
      if (title != null) {
        graph += '---\n';
        graph += 'title: ';
        graph += title;
        graph += '\n---\n';
      }
      graph += Children.onlyText(children);
      mermaidAPI.initialize({
        theme: 'dark',
      });
      const id = "id" + Math.random().toString(16).slice(2)
      const { svg, bindFunctions } = await mermaidAPI.render(id, graph);
      setDiagram(svg);
    }
    if (diagram == null) {
      renderDiagram();
    }
  }, []);

  return (
    <div dangerouslySetInnerHTML={{__html: diagram}} />
  )
}

export default Mermaid;
