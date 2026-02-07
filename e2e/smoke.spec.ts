import { expect, test } from "@playwright/test";
import { GamePage } from "./fixtures";

test.describe("LoreKeeper Smoke Tests", () => {
  test("title screen loads with heading and buttons", async ({ page }) => {
    const game = new GamePage(page);
    await game.goto();

    await expect(game.titleHeading).toContainText("Thornhold");
    await expect(game.newGameButton).toBeVisible();
    await expect(game.loadGameButton).toBeVisible();
    await expect(game.settingsButton).toBeVisible();
  });

  test("new game starts and shows terminal", async ({ page }) => {
    const game = new GamePage(page);
    await game.goto();

    await game.newGameButton.click();

    await expect(game.commandInput).toBeVisible();
    await expect(game.gameOutput).toBeVisible();
  });

  test("typing look produces output", async ({ page }) => {
    const game = new GamePage(page);
    await game.goto();
    await game.newGameButton.click();

    await expect(game.commandInput).toBeVisible();
    await game.typeCommand("look");

    // Should see location description in output
    await expect(game.gameOutput).toContainText("Courtyard", {
      timeout: 5_000,
    });
  });

  test("settings overlay opens and closes with Escape", async ({ page }) => {
    const game = new GamePage(page);
    await game.goto();

    await game.settingsButton.click();

    const dialog = page.getByRole("dialog");
    await expect(dialog).toBeVisible();

    await page.keyboard.press("Escape");
    await expect(dialog).not.toBeVisible();
  });

  test("save/load screen accessible from title", async ({ page }) => {
    const game = new GamePage(page);
    await game.goto();

    await game.loadGameButton.click();

    const dialog = page.getByRole("dialog");
    await expect(dialog).toBeVisible();
    await expect(dialog).toContainText("Load Game");
  });
});
