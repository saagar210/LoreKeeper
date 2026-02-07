import type { Page } from "@playwright/test";

export class GamePage {
  constructor(public page: Page) {}

  async goto() {
    await this.page.goto("/");
  }

  get titleHeading() {
    return this.page.locator("h1");
  }

  get newGameButton() {
    return this.page.getByRole("button", { name: "New Game" });
  }

  get loadGameButton() {
    return this.page.getByRole("button", { name: "Load Game" });
  }

  get settingsButton() {
    return this.page.getByRole("button", { name: "Settings" });
  }

  get commandInput() {
    return this.page.getByLabel("Game command input");
  }

  get gameOutput() {
    return this.page.getByLabel("Game output");
  }

  async typeCommand(cmd: string) {
    await this.commandInput.fill(cmd);
    await this.commandInput.press("Enter");
  }
}
