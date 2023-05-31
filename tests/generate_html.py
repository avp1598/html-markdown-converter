# generate 50MB big html file

html_content = """
    <h1>Heading 1</h1>

    <h2>Heading 2</h2>

    <h3>Heading 3</h3>

    <strong>Bold Text</strong>

    <div src="https://example.com/image.png" height="12" width="12">Image</div>

    <em>Italic Text</em>

    <a href="https://example.com">Link</a>

    <ul>
        <li>List Item 1</li>
        <li>List Item 2
            <ul>
                <li>Nested List Item 2.1</li>
                <li>Nested List Item 2.2</li>
                <li>Nested List Item 2.3</li>
            </ul>
        </li>
        <li>List Item 3</li>
    </ul>

    <ol>
        <li>Ordered Item 1</li>
        <li>Ordered Item 2
            <ol>
                <li>Nested Ordered Item 2.1</li>
                <li>Nested Ordered Item 2.2</li>
                <li>Nested Ordered Item 2.3</li>
            </ol>
        </li>
        <li>Ordered Item 3</li>
    </ol>

    <blockquote>Blockquote</blockquote>

    <pre><code>print("Hello, World!")</code></pre>

    <table>
        <tr>
            <th>Column 1</th>
            <th>Column 2</th>
        </tr>
        <tr>
            <td>Cell 1</td>
            <td>Cell 2</td>
        </tr>
        <tr>
            <td>Cell 3</td>
            <td>Cell 4</td>
        </tr>
    </table>
"""

with open("output.html", "w") as f:
    while f.tell() < 50 * 1024 * 1024:
        f.write(html_content)
