{-# LANGUAGE OverloadedStrings #-}

import Control.Monad (forM_)
import Data.Maybe (fromMaybe)
import qualified Data.Text as T
import Hakyll
import Slug (toSlug)
import Text.Pandoc
  ( Extension (Ext_fenced_code_attributes, Ext_footnotes, Ext_gfm_auto_identifiers, Ext_implicit_header_references, Ext_smart),
    Extensions,
    ReaderOptions,
    WriterOptions,
    extensionsFromList,
    githubMarkdownExtensions,
    readerExtensions,
    writerExtensions,
  )

-- CONFIG

root :: String
root =
  "https://ramda.guide"

siteName :: String
siteName =
  "Ramda Ramp-Up Guide"

config :: Configuration
config =
  defaultConfiguration
    { destinationDirectory = "../dist",
      ignoreFile = const False,
      previewHost = "127.0.0.1",
      previewPort = 8000,
      providerDirectory = "../src",
      storeDirectory = "../hakyll-cache",
      tmpDirectory = "../hakyll-cache/tmp"
    }

-- BUILD

{- ORMOLU_DISABLE -}

main :: IO ()
main = hakyllWith config $ do
  forM_
    [ "CNAME",
      "favicon.ico",
      "robots.txt",
      "_config.yml",
      "images/*",
      "js/*",
      "fonts/*"
    ] $ \f -> match f $ do
    route idRoute
    compile copyFileCompiler

  match "css/*" $ do
    route idRoute
    compile compressCssCompiler

  match "posts/*" $ do
    let ctx = constField "type" "article" <> postCtx
    route $ metadataRoute titleRoute
    compile $
      pandocCompilerCustom
        >>= loadAndApplyTemplate "templates/post.html" ctx
        >>= saveSnapshot "content"
        >>= loadAndApplyTemplate "templates/default.html" ctx

  match "index.html" $ do
    route idRoute
    compile $ do
      posts <- recentFirst =<< loadAll "posts/*"
      let indexCtx =
            listField "posts" postCtx (return posts)
              <> constField "root" root
              <> constField "siteName" siteName
              <> defaultContext
      getResourceBody
        >>= applyAsTemplate indexCtx
        >>= loadAndApplyTemplate "templates/default.html" indexCtx

  match "templates/*" $ compile templateBodyCompiler

  create ["sitemap.xml"] $ do
    route idRoute
    compile $ do
      posts <- recentFirst =<< loadAll "posts/*"
      let pages = posts
          sitemapCtx =
            constField "root" root
              <> constField "siteName" siteName
              <> listField "pages" postCtx (return pages)
      makeItem ("" :: String)
        >>= loadAndApplyTemplate "templates/sitemap.xml" sitemapCtx

    create ["rss.xml"] $ do
      route idRoute
      compile (feedCompiler renderRss)

    create ["atom.xml"] $ do
      route idRoute
      compile (feedCompiler renderAtom)

{- ORMOLU_ENABLE -}

-- CONTEXT

feedCtx :: Context String
feedCtx =
  titleCtx
    <> postCtx
    <> bodyField "description"


postCtx :: Context String
postCtx =
  constField "root" root
    <> constField "siteName" siteName
    <> dateField "date" "%Y-%m-%d"
    <> defaultContext

titleCtx :: Context String
titleCtx =
  field "title" updatedTitle

-- TITLE HELPERS

replaceAmp :: String -> String
replaceAmp =
  replaceAll "&" (const "&amp;")

replaceTitleAmp :: Metadata -> String
replaceTitleAmp =
  replaceAmp . safeTitle

safeTitle :: Metadata -> String
safeTitle =
  fromMaybe "no title" . lookupString "title"

updatedTitle :: Item a -> Compiler String
updatedTitle =
  fmap replaceTitleAmp . getMetadata . itemIdentifier

-- PANDOC

pandocCompilerCustom :: Compiler (Item String)
pandocCompilerCustom =
  pandocCompilerWith pandocReaderOpts pandocWriterOpts

pandocExtensionsCustom :: Extensions
pandocExtensionsCustom =
  githubMarkdownExtensions
    <> extensionsFromList
      [ Ext_fenced_code_attributes,
        Ext_gfm_auto_identifiers,
        Ext_implicit_header_references,
        Ext_smart,
        Ext_footnotes
      ]

pandocReaderOpts :: ReaderOptions
pandocReaderOpts =
  defaultHakyllReaderOptions
    { readerExtensions = pandocExtensionsCustom
    }

pandocWriterOpts :: WriterOptions
pandocWriterOpts =
  defaultHakyllWriterOptions
    { writerExtensions = pandocExtensionsCustom
    }

-- FEEDS

type FeedRenderer =
  FeedConfiguration ->
  Context String ->
  [Item String] ->
  Compiler (Item String)

feedCompiler :: FeedRenderer -> Compiler (Item String)
feedCompiler renderer =
  renderer feedConfiguration feedCtx
    =<< recentFirst
    =<< loadAllSnapshots "posts/*" "content"

feedConfiguration :: FeedConfiguration
feedConfiguration =
  FeedConfiguration
    { feedTitle = "Ramda Ramp-Up Guide",
      feedDescription = "An in-depth guide to help get you mostly adequate with ramda.js",
      feedAuthorName = "Robert W. Pearce",
      feedAuthorEmail = "me@robertwpearce.com",
      feedRoot = root
    }

-- CUSTOM ROUTE

getTitleFromMeta :: Metadata -> String
getTitleFromMeta =
  fromMaybe "no title" . lookupString "title"

fileNameFromTitle :: Metadata -> FilePath
fileNameFromTitle =
  T.unpack . (`T.append` ".html") . toSlug . T.pack . getTitleFromMeta

titleRoute :: Metadata -> Routes
titleRoute =
  constRoute . fileNameFromTitle
