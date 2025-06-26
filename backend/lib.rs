<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Hello World</title>
    <style>
      body {
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
        background: linear-gradient(135deg, #dbeafe, #f0fdf4);
        margin: 0;
        padding: 0;
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100vh;
      }

      main {
        background-color: #ffffffcc;
        padding: 40px;
        border-radius: 16px;
        box-shadow: 0 12px 24px rgba(0, 0, 0, 0.15);
        max-width: 500px;
        width: 100%;
        text-align: center;
        backdrop-filter: blur(8px);
      }

      form {
        display: flex;
        flex-direction: column;
        gap: 20px;
      }

      label {
        font-size: 1.2rem;
        color: #333;
      }

      input[type='text'] {
        padding: 10px;
        font-size: 1rem;
        border: 1px solid #ccc;
        border-radius: 8px;
        outline: none;
      }

      button {
        padding: 10px;
        font-size: 1rem;
        background-color: #4f46e5;
        color: white;
        border: none;
        border-radius: 8px;
        cursor: pointer;
        transition: background-color 0.3s ease;
      }

      button:hover {
        background-color: #4338ca;
      }

      #greeting {
        margin-top: 20px;
        padding: 15px;
        font-size: 1.1rem;
        color: #155724;
        background-color: #d4edda;
        border: 1px solid #c3e6cb;
        border-radius: 8px;
      }

      #greeting:empty {
        display: none;
      }
    </style>
    <script type="module">
      import { backend } from 'declarations/backend';
      document.querySelector('form').addEventListener('submit', async (e) => {
        e.preventDefault();
        const name = document.getElementById('name').value.toString();
        const greeting = await backend.greet(name);
        document.getElementById('greeting').innerText = greeting;
      });
    </script>
  </head>
  <body>
    <main>
      <form>
        <label for="name">I am Alex. Enter your name:</label>
        <input id="name" alt="Name" type="text" placeholder="Your name..." />
        <button type="submit">Click Me!</button>
      </form>
      <section id="greeting"></section>
    </main>
  </body>
</html>
