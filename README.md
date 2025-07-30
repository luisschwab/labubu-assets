# Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder.
If you chose to develop with the router feature, you will also have a `views` folder.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # The entrypoint for the app. It also defines the routes for the app.
│  ├─ components/
│  │  ├─ mod.rs # Defines the components module
│  │  ├─ hero.rs # The Hero component for use in the home page
│  ├─ views/ # The views each route will render in the app.
│  │  ├─ mod.rs # Defines the module for the views route and re-exports the components for each route
│  │  ├─ blog.rs # The component that will render at the /blog/:id route
│  │  ├─ home.rs # The component that will render at the / route
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

## Deployment

### GitHub Pages

This project is configured for automatic deployment to GitHub Pages. The deployment process is handled by GitHub Actions.

#### Setup

1. **Enable GitHub Pages**: Go to your repository settings → Pages → Source → Select "Deploy from a branch" → Choose "gh-pages" branch → Save

2. **Push to main branch**: The GitHub Actions workflow will automatically:
   - Build your Dioxus application
   - Deploy it to the `gh-pages` branch
   - Make it available at `https://yourusername.github.io/your-repo-name`

#### Manual Build

To build locally for deployment:

```bash
just build
```

This will create a `docs` directory with the built application ready for deployment.

#### Custom Domain (Optional)

If you want to use a custom domain:
1. Add your domain to the GitHub Pages settings
2. Create a `CNAME` file in the `docs` directory with your domain name
3. The workflow will automatically include this file in the deployment

