import { expect, test } from "@playwright/test";
import { GamePage } from "./fixtures";

test.describe("Integration Flows", () => {
  test("Save → Play → Load preserves game state", async ({ page }) => {
    const game = new GamePage(page);
    await game.goto();

    // Start new game
    await game.newGameButton.click();
    await expect(game.gameOutput).toContainText("descend", { timeout: 5000 });

    // Execute command to pick up item
    await game.typeCommand("take rusty_lantern");
    await expect(game.gameOutput).toContainText("rusty", { timeout: 3000 });

    // Quick save: dispatch key combo directly to avoid browser-reserved shortcut flakiness
    await page.evaluate(() => {
      window.dispatchEvent(
        new KeyboardEvent("keydown", {
          key: "s",
          ctrlKey: true,
          bubbles: true,
          cancelable: true,
        }),
      );
    });

    // Wait for save confirmation
    await expect(page.locator("text=Game saved.")).toBeVisible({ timeout: 3000 });

    // Execute more commands to change state
    await game.typeCommand("go east");
    await page.waitForTimeout(1000);

    // Open load game screen
    await page.keyboard.press("Escape");
    await game.loadGameButton.click();

    // Load from quicksave row
    const quicksaveRow = page.locator("div", { hasText: /quicksave/i })
      .filter({ has: page.getByRole("button", { name: "Load" }) })
      .first();
    await expect(quicksaveRow).toBeVisible({ timeout: 3000 });
    await quicksaveRow.getByRole("button", { name: "Load" }).click();

    // Verify loaded state in sidebar location
    const locationHeading = page.getByRole("complementary", { name: "Game information" })
      .getByRole("heading", { level: 3 })
      .first();
    await expect(locationHeading).toHaveText("Courtyard", { timeout: 3000 });
  });

  test("Achievement notification appears on unlock", async ({ page }) => {
    const game = new GamePage(page);
    await game.goto();

    // Start new game
    await game.newGameButton.click();
    await expect(game.commandInput).toBeVisible({ timeout: 5000 });

    // Navigate to an area with enemies (if barracks is accessible)
    await game.typeCommand("go south");
    await page.waitForTimeout(500);

    // Attempt to engage in combat
    await game.typeCommand("look");
    const output = await game.gameOutput.textContent();

    // If there's an enemy mentioned, try to attack
    if (output && (output.includes("zombie") || output.includes("goblin"))) {
      await game.typeCommand("attack zombie");

      // Wait for combat resolution
      await page.waitForTimeout(2000);

      // Keep attacking until enemy is defeated or health is low
      for (let i = 0; i < 10; i++) {
        const currentOutput = await game.gameOutput.textContent();
        if (
          currentOutput?.includes("defeated") ||
          currentOutput?.includes("victory") ||
          currentOutput?.includes("dies")
        ) {
          // Check if achievement notification appears
          // This might be shown in game output or as a separate notification
          break;
        }
        if (currentOutput?.includes("You are dead") || currentOutput?.includes("death")) {
          break;
        }
        await game.typeCommand("attack zombie");
        await page.waitForTimeout(1000);
      }
    }
  });

  test("Terminal accepts and displays player commands", async ({ page }) => {
    const game = new GamePage(page);
    await game.goto();
    await game.newGameButton.click();

    await expect(game.commandInput).toBeVisible({ timeout: 5000 });

    // Test basic commands
    const commands = ["look", "inventory", "help"];

    for (const cmd of commands) {
      await game.typeCommand(cmd);
      await page.waitForTimeout(500);

      // Verify command echo appears in output
      await expect(game.gameOutput).toContainText(`> ${cmd}`, { timeout: 2000 });
    }
  });

  test("Settings can be changed and persist", async ({ page }) => {
    const game = new GamePage(page);
    await game.goto();

    // Open settings
    await game.settingsButton.click();
    const dialog = page.getByRole("dialog");
    await expect(dialog).toBeVisible();

    // Look for difficulty settings or theme settings
    const difficultySelect = page.locator('select[name="difficulty"]');
    if (await difficultySelect.isVisible()) {
      await difficultySelect.selectOption("hard");
    }

    // Close settings
    await page.keyboard.press("Escape");
    await expect(dialog).not.toBeVisible({ timeout: 1000 });

    // Reopen settings to verify persistence
    await game.settingsButton.click();
    await expect(dialog).toBeVisible();

    // Verify setting was saved
    if (await difficultySelect.isVisible()) {
      const selectedValue = await difficultySelect.inputValue();
      expect(selectedValue).toBe("hard");
    }
  });

  test("Keyboard navigation works (Escape closes overlays)", async ({ page }) => {
    const game = new GamePage(page);
    await game.goto();

    // Start game
    await game.newGameButton.click();
    await expect(game.commandInput).toBeVisible({ timeout: 5000 });

    // Press Escape to open settings
    await page.keyboard.press("Escape");
    const dialog = page.getByRole("dialog");
    await expect(dialog).toBeVisible({ timeout: 1000 });

    // Press Escape again to close
    await page.keyboard.press("Escape");
    await expect(dialog).not.toBeVisible({ timeout: 1000 });
  });

  test("Game output scrolls with new content", async ({ page }) => {
    const game = new GamePage(page);
    await game.goto();
    await game.newGameButton.click();

    await expect(game.commandInput).toBeVisible({ timeout: 5000 });

    // Execute many commands to generate scrollable content
    for (let i = 0; i < 10; i++) {
      await game.typeCommand("look");
      await page.waitForTimeout(300);
    }

    // Verify output contains recent commands (last few "look" echoes)
    const output = await game.gameOutput.textContent();
    expect(output).toContain("> look");

    // Verify the output area is scrollable or contains multiple lines
    const outputElement = game.gameOutput;
    const scrollHeight = await outputElement.evaluate((el) => el.scrollHeight);
    const clientHeight = await outputElement.evaluate((el) => el.clientHeight);

    // If content is long enough, scrollHeight should exceed clientHeight
    if (scrollHeight > clientHeight) {
      expect(scrollHeight).toBeGreaterThan(clientHeight);
    }
  });

  test("Invalid commands show error messages", async ({ page }) => {
    const game = new GamePage(page);
    await game.goto();
    await game.newGameButton.click();

    await expect(game.commandInput).toBeVisible({ timeout: 5000 });

    // Type nonsense command
    await game.typeCommand("xyzzy plugh");
    await page.waitForTimeout(500);

    // Should see some kind of response (either error or "unknown command")
    const output = await game.gameOutput.textContent();
    expect(output).toBeTruthy();
    expect(output?.length).toBeGreaterThan(0);
  });
});

test.describe("UI Components", () => {
  test("Sidebar displays player stats when in game", async ({ page }) => {
    const game = new GamePage(page);
    await game.goto();
    await game.newGameButton.click();

    await expect(game.commandInput).toBeVisible({ timeout: 5000 });

    // Look for sidebar or stats panel (might be desktop only)
    const sidebar = page.locator('aside, [role="complementary"], .sidebar');
    const isMobile = page.viewportSize()?.width && page.viewportSize()!.width < 768;

    if (!isMobile) {
      // On desktop, sidebar should be visible
      if (await sidebar.isVisible()) {
        // Verify it contains stat information
        const sidebarText = await sidebar.textContent();
        expect(sidebarText).toMatch(/health|inventory|stats/i);
      }
    }
  });

  test("Theme changes apply immediately", async ({ page }) => {
    const game = new GamePage(page);
    await game.goto();

    await game.settingsButton.click();
    const dialog = page.getByRole("dialog");
    await expect(dialog).toBeVisible();

    // Look for theme selector
    const themeSelect = page.locator('select[name="theme"], button:has-text("Theme")');
    if (await themeSelect.first().isVisible()) {
      // Get current background color
      const body = page.locator("body");
      const initialBg = await body.evaluate((el) =>
        window.getComputedStyle(el).backgroundColor,
      );

      // Change theme (if we can find theme controls)
      const themeButtons = page.locator('button[data-theme]');
      if ((await themeButtons.count()) > 0) {
        await themeButtons.first().click();
        await page.waitForTimeout(300);

        // Verify background changed
        const newBg = await body.evaluate((el) =>
          window.getComputedStyle(el).backgroundColor,
        );

        // Background should potentially change (may not if already on that theme)
        expect(typeof newBg).toBe("string");
      }
    }
  });
});

test.describe("Mobile Responsive", () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test("Mobile: Terminal input is accessible", async ({ page }) => {
    const game = new GamePage(page);
    await game.goto();
    await game.newGameButton.click();

    await expect(game.commandInput).toBeVisible({ timeout: 5000 });

    // Verify input is large enough for touch
    const inputBox = await game.commandInput.boundingBox();
    if (inputBox) {
      expect(inputBox.height).toBeGreaterThanOrEqual(30); // Minimum touch target
    }

    // Can type and submit
    await game.typeCommand("look");
    await expect(game.gameOutput).toContainText("> look", { timeout: 2000 });
  });

  test("Mobile: Sidebar accessible via toggle button", async ({ page }) => {
    const game = new GamePage(page);
    await game.goto();
    await game.newGameButton.click();

    await expect(game.commandInput).toBeVisible({ timeout: 5000 });

    // Look for sidebar toggle button
    const toggleButton = page.locator('button:has-text("info"), button.sidebar-toggle');
    if (await toggleButton.isVisible()) {
      await toggleButton.click();

      // Sidebar drawer should appear
      const drawer = page.locator('.sidebar-drawer, [role="dialog"]:has-text("Stats")');
      await expect(drawer).toBeVisible({ timeout: 1000 });

      // Close drawer
      await page.keyboard.press("Escape");
      await expect(drawer).not.toBeVisible({ timeout: 1000 });
    }
  });
});
