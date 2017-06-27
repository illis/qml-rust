/*
 * Copyright (C) 2016 Lucien XU <sfietkonstantin@free.fr>
 *
 * You may use this file under the terms of the BSD license as follows:
 *
 * "Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in
 *     the documentation and/or other materials provided with the
 *     distribution.
 *   * The names of its contributors may not be used to endorse or promote
 *     products derived from this software without specific prior written
 *     permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE."
 */

#include "dothersideextra.h"
#include "deqlistmodel.h"
#include "deqlistmodelmetaobject.h"
#include "deqml.h"
#include "deqobject.h"
#include "idedobjectcontainer.h"
#include <DOtherSide/DosQMetaObject.h>
#include <algorithm>
#include <cstring>
#include <memory>
#include <vector>

#ifdef SAILFISH
#include <sailfishapp.h>
#else
#include <QGuiApplication>
#include <QQmlEngine>
#include <QQuickView>
#endif

static QGuiApplication *createGuiApplication(int &argc, char **argv)
{
#ifdef SAILFISH
    return SailfishApp::application(argc, argv);
#else
    return new QGuiApplication{argc, argv};
#endif
}

static QQuickView *createQuickView()
{
#ifdef SAILFISH
    return SailfishApp::createView();
#else
    QQuickView *view{new QQuickView{}};
    QObject::connect(view->engine(), &QQmlEngine::quit, QCoreApplication::instance(), &QCoreApplication::quit);
    return view;
#endif
}

template <typename T>
struct WrappedArray
{
    WrappedArray(const T *first, std::ptrdiff_t size)
        : m_begin{first}
        , m_end{first + size}
    {
    }
    const T *begin() const noexcept
    {
        return m_begin;
    }
    const T *end() const noexcept
    {
        return m_end;
    }

    const T *m_begin;
    const T *m_end;
};

class DEApplicationImpl
{
public:
    explicit DEApplicationImpl(int argc, const char *const *argv)
        : m_argc{argc}
    {
        const WrappedArray<const char *> array{argv, static_cast<std::ptrdiff_t>(argc)};
        std::transform(array.begin(), array.end(), std::back_inserter(m_arguments),
                       [](const char *string) { return QByteArray{string}; });
        std::transform(m_arguments.begin(), m_arguments.end(), std::back_inserter(m_argv),
                       [](QByteArray &stringPtr) { return stringPtr.data(); });

        m_application.reset(createGuiApplication(m_argc, m_argv.data()));
    }
    QGuiApplication &application() const
    {
        return *m_application;
    }

private:
    std::unique_ptr<QGuiApplication> m_application{};
    std::vector<QByteArray> m_arguments{};
    int m_argc{0};
    std::vector<char *> m_argv{};
};

template <class T>
static T fromDeQvariantMap(const DEQVariantMap *value)
{
    T returned{};
    for (int i = 0; i < value->count; ++i) {
        auto entry = value->values[i];
        returned[QString::fromLocal8Bit(entry.key)] = *(static_cast<const QVariant *>(entry.value));
    }
    return returned;
}

template <class T>
class IteratorProvider;

template <>
class IteratorProvider<QVariantMap>
{
public:
    static QVariantMap::ConstIterator begin(const QVariantMap &value)
    {
        return value.constBegin();
    }
    static QVariantMap::ConstIterator end(const QVariantMap &value)
    {
        return value.constEnd();
    }
    static QString key(const QVariantMap::ConstIterator &it)
    {
        return it.key();
    }
    static QVariant value(const QVariantMap::ConstIterator &it)
    {
        return it.value();
    }
};

template <>
class IteratorProvider<std::map<QString, QVariant>>
{
public:
    static std::map<QString, QVariant>::const_iterator begin(const std::map<QString, QVariant> &value)
    {
        return value.begin();
    }
    static std::map<QString, QVariant>::const_iterator end(const std::map<QString, QVariant> &value)
    {
        return value.end();
    }
    static QString key(const std::map<QString, QVariant>::const_iterator &it)
    {
        return it->first;
    }
    static QVariant value(const std::map<QString, QVariant>::const_iterator &it)
    {
        return it->second;
    }
};

template <class T>
static DEQVariantMap *toDeQVariantMap(const T &value)
{
    DEQVariantMap *returned = new DEQVariantMap{static_cast<int>(value.size()), new DEQVariantMapEntry[value.size()]};

    std::size_t index = 0;
    for (auto it = IteratorProvider<T>::begin(value); it != IteratorProvider<T>::end(value); ++it) {
        auto key{IteratorProvider<T>::key(it)};
        char *keyPtr = new char[key.size() + 1]{0};
        std::strncpy(keyPtr, key.toLocal8Bit().data(), static_cast<std::size_t>(key.size() + 1));
        returned->values[index].key = keyPtr;
        returned->values[index].value = new QVariant{IteratorProvider<T>::value(it)};
        ++index;
    }

    return returned;
}

static DOS::QmlRegisterType fromRawQmlRegisterType(const QmlRegisterType *qmlRegisterType)
{
    auto holder = static_cast<DOS::DosIQMetaObjectHolder *>(qmlRegisterType->staticMetaObject);

    DOS::QmlRegisterType returned;
    returned.major = qmlRegisterType->major;
    returned.minor = qmlRegisterType->minor;
    returned.uri = qmlRegisterType->uri;
    returned.qml = qmlRegisterType->qml;
    returned.staticMetaObject = holder->data();
    returned.createDObject = qmlRegisterType->createDObject;
    returned.deleteDObject = qmlRegisterType->deleteDObject;

    return returned;
}

DEApplication *de_qguiapplication_create(int argc, const char *const *argv)
{
    return new DEApplicationImpl{argc, argv};
}

void de_qguiapplication_delete(DEApplication *vptr)
{
    delete static_cast<DEApplicationImpl *>(vptr);
}

DosQQuickView *de_qquickview_create()
{
    return createQuickView();
}

void de_qquickview_set_source_url(DosQQuickView *vptr, const DosQUrl *url)
{
    auto view = static_cast<QQuickView *>(vptr);
    auto _url = static_cast<const QUrl *>(url);
    view->setSource(*_url);
}

DEObject *de_qobject_create(const DosQMetaObject *metaObject, DObjectCallback dObjectCallback)
{
    auto metaObjectHolder = static_cast<const DOS::DosIQMetaObjectHolder *>(metaObject);
    auto deQObject = new DEQObject(metaObjectHolder->data(), dObjectCallback);
    QQmlEngine::setObjectOwnership(deQObject, QQmlEngine::CppOwnership);
    return static_cast<QObject *>(deQObject);
}

void de_qobject_set_dobject(DEObject *vptr, void *dObject)
{
    auto deQObject = static_cast<DEQObject *>(vptr);
    deQObject->setDObject(dObject);
}

void *de_qobject_check_and_get_dobject(DEObject *vptr, const DosQMetaObject *meta)
{
    auto qobject = static_cast<QObject *>(vptr);
    auto objectContainer = dynamic_cast<IDeDObjectContainer *>(qobject);
    if (objectContainer == nullptr) {
        return nullptr;
    }

    auto currentMetaObject = qobject->metaObject();
    auto holder = static_cast<const DOS::DosIQMetaObjectHolder *>(meta);
    auto metaObject = holder->data()->metaObject();
    if (std::string(metaObject->className()) == std::string(currentMetaObject->className())) {
        return objectContainer->dObject();
    } else {
        return nullptr;
    }
}

DosQMetaObject *de_qlistmodel_qmetaobject()
{
    return new DOS::DosIQMetaObjectHolder(std::make_shared<DEQListModelMetaObject>());
}

DEListModel *de_qlistmodel_create(const DosQMetaObject *metaObject, const char *const *roleArray, int roleArrayLength,
                                  DObjectCallback dObjectCallback)
{
    std::map<int, QByteArray> roleNames{};
    for (int i = 0; i < roleArrayLength; ++i) {
        const char *roleName{roleArray[i]};
        roleNames[Qt::UserRole + i] = QByteArray{roleName};
    }

    auto metaObjectHolder = static_cast<const DOS::DosIQMetaObjectHolder *>(metaObject);
    auto model = new DEQListModel(metaObjectHolder->data(), std::move(roleNames), dObjectCallback);
    QQmlEngine::setObjectOwnership(model, QQmlEngine::CppOwnership);
    return static_cast<QObject *>(model);
}

void de_qlistmodel_set_dobject(DEListModel *vptr, void *dObject)
{
    auto deListModel = static_cast<DEQListModel *>(vptr);
    deListModel->setDObject(dObject);
}

int de_qlistmodel_count(const DEListModel *vptr)
{
    auto deListModel = static_cast<const DEQListModel *>(vptr);
    return deListModel->count();
}

void de_qlistmodel_insert(DEListModel *vptr, int row, const DEQvariantMapList *values)
{
    auto deListModel = static_cast<DEQListModel *>(vptr);
    std::vector<DEQBaseListModel::Data> entries{};
    for (int i = 0; i < values->count; ++i) {
        entries.push_back(
            deListModel->fromKeyValue(fromDeQvariantMap<std::map<QString, QVariant>>(&values->values[i])));
    }
    deListModel->insert(row, std::move(entries));
}

void de_qlistmodel_remove(DEListModel *vptr, int row, int count)
{
    auto deListModel = static_cast<DEQListModel *>(vptr);
    deListModel->remove(row, count);
}

DEQVariantMap *de_qlistmodel_get(const DEListModel *vptr, int index)
{
    auto deListModel = static_cast<const DEQListModel *>(vptr);
    if (index < 0 || index >= deListModel->count()) {
        return nullptr;
    }

    auto item = deListModel->toKeyValue(deListModel->get(index));
    return toDeQVariantMap<std::map<QString, QVariant>>(item);
}

int de_qqml_qmlregisterobject(const QmlRegisterType *qmlRegisterType)
{
    return deQmlRegisterQObject(fromRawQmlRegisterType(qmlRegisterType));
}

DosQVariant *de_qvariant_create_qvariantmap(const DEQVariantMap *value)
{
    return new QVariant{fromDeQvariantMap<QVariantMap>(value)};
}

DEQVariantMap *de_qvariant_to_qvariantmap(const DosQVariant *vptr)
{
    auto variant = static_cast<const QVariant *>(vptr);
    auto map = variant->toMap();

    return toDeQVariantMap<QVariantMap>(map);
}

void de_qvariantmap_delete(const DEQVariantMap *vptr)
{
    for (int i = 0; i < vptr->count; ++i) {
        delete[] vptr->values[i].key;
        delete static_cast<const QVariant *>(vptr->values[i].value);
    }
    delete[] vptr->values;
    delete vptr;
}
