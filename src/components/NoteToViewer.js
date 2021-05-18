import React from "react";
import clsx from "clsx";
import styles from "./Note.css";

export default function NoteToViewer() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="text--center padding-horiz--md">
          <br /> <br /> <br />
          <h2>
            {" "}
            🚧 Note: This Site is a WIP! 🚧
          </h2>
          <h3>Contributions and feedback welcome! Please submit an
              <a href=" https://github.com/substrate-developer-hub/substrate-how-to-guides/">
              {" "}
              Issue or PR{" "}
              </a>{" "}
              😃
          </h3>
          <p>
            <strong>Prelude:</strong> Substrate is fundamentally about composing the included{" "}
            <a href="https://substrate.dev/docs/en/knowledgebase/runtime/primitives">
              {" "}
              blockchain primitives{" "}
            </a>{" "}
            into <em>runtime</em> and <em>node</em> logic to produce robust
            blockchains and parachains.
          </p>
          <p>
            The <strong>Substrate how-to guide book</strong> centers on rapidly integrating
            and using capabilities <em>modularly</em> for Substrate based blockchains.
          </p>
        </div>
      </div>
    </section>
  );
}